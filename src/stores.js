import { writable } from 'svelte/store';

// Global Variables (modal toggle)
export const success = writable('');
export const error = writable({});

export const newProject = writable(false);

// App Config Group
export const CONFIG = writable({});

// Contents Group
export const projectList = writable(new Set());
export const contents = writable([]);

export const initConfig = (data) => {
  const { current_project, theme, export_path, export_format } = data;

  CONFIG.set({
    theme,
    currentProject: current_project,
    exportPath: export_path,
    exportFormat: export_format,
  });
};
