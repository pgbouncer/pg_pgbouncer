use crate::*;

#[derive(Default)]
pub struct ManagerState {
    pub groups: BTreeMap<i32, Group>,
    pub previous_groups: Option<BTreeMap<i32, Group>>,
    pub children: DefaultBTreeMap<i32, Vec<PgBouncerChild>>,
}

impl ManagerState {
    pub fn do_main_loop(&mut self) -> Result<()> {
        if BackgroundWorker::sighup_received() {
            // on SIGHUP, you might want to reload some external configuration or something
        }
        if BackgroundWorker::sigint_received() {
            panic!("ASKED TO QUIT")
        }
        self.groups = BackgroundWorker::transaction(Group::all)?;
        self.create_all()?;
        self.cleanup_old_files()?;
        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<()> {
        for group in self.groups.clone().values() {
            if !group.stay_alive {
                self.stop_group(group)?;
            }
        }
        Ok(())
    }

    fn stop_group(&mut self, group: &Group) -> Result<()> {
        let children = self.children.get_mut(group.id);
        for child in children.iter_mut() {
            child.stop()?;
        }
        Ok(())
    }

    fn create_all(&mut self) -> Result<()> {
        for group in self.groups.clone().values() {
            self.update_group(group)?;
        }
        Ok(())
    }

    fn cleanup_old_files(&self) -> Result<()> {
        if !self.changed() {
            return Ok(());
        }

        for dir in fs::read_dir(BASE_DIR)? {
            let dir = dir?;
            let path = dir.path();
            // The base directory should only contain directories, remove anything else
            if !path.is_dir() {
                log!("Removing file {}", path.to_string_lossy());
                fs::remove_file(&path)?;
                continue;
            }

            let dir_name: Option<&str> = path.file_name().and_then(|fname| fname.to_str());

            // We want to keep the .tmp directory around
            if dir_name == Some(".tmp") {
                continue;
            }

            // The base directory should only contain .tmp and ones with a numberic name
            let group_id_opt = dir_name.and_then(|s| s.parse::<i32>().ok());
            let Some(group_id) = group_id_opt else {
                log!("Removing directory {}", path.to_string_lossy());
                fs::remove_dir_all(&path)?;
                continue;
            };

            // We remove directories for unknown groups
            let group_opt = self.groups.get(&group_id);
            let Some(group) = group_opt else {
                log!("Removing directory {}", path.to_string_lossy());
                fs::remove_dir_all(&path)?;
                continue;
            };

            group.cleanup_old_files()?;
        }

        Ok(())
    }

    fn changed(&self) -> bool {
        self.previous_groups.as_ref() != Some(&self.groups)
    }

    fn update_group(&mut self, group: &Group) -> Result<()> {
        let changed = group.changed(self);
        let should_restart = group.should_restart()?;

        let children = self.children.get_mut(group.id);
        if changed {
            group.update_children(children)?;

            group.create_shared_configs()?;

            for child in children.iter() {
                child.process.create_config()?;
            }

            for child in children.iter_mut() {
                child.reload()?;
            }
        }
        group.write_state()?;

        for child in children {
            if should_restart {
                child.stop()?;
            }
            child.start()?;
        }

        group.update_restart_count()?;

        Ok(())
    }
}
