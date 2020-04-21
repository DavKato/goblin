import { CONFIG, projectList, newProject } from '../stores';
import { messenger } from './messenger';

export const createProject = (name) => {
  projectList.update((projects) => projects.add(name));

  CONFIG.set({ ...CONFIG, currentProject: name });

  messenger(
    {
      cmd: 'createProject',
      project_name: name,
    },
    true,
  );

  newProject.set(false);
};
