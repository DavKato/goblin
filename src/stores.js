import { writable } from 'svelte/store';

// Global Variables (modal toggle)
export const success = writable('');
export const error = writable('');

export const newProject = writable(false);

// App Config Group
export const CONFIG = writable({});
export const currentProject = writable('');

// Contents Group
export const projectList = writable([]);
export const contents = writable([]);

export const initConfig = (data) => {
  console.log(data);

  const { current_project, theme, exportPath, exportFormat } = data;

  currentProject.set(current_project);

  CONFIG.set({
    theme,
    exportPath,
    exportFormat
  });
};
