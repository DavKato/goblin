import { currentProject, projectList, newProject } from '../stores';
import { messenger } from './messenger';

export const createProject = (name) => {
  console.log(name);

  projectList.update((projects) => projects.push(name));
  currentProject.set(name);

  messenger(
    {
      cmd: 'createProject',
      project_name: name
    },
    true
  );

  newProject.set(false);
};
