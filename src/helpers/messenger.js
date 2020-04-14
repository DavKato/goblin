import { error, success } from '../stores';

export const messenger = async (payload, informSuccess = false) => {
  try {
    const res = await window.tauri.promisified(payload);

    if (informSuccess) {
      const message = res.message || res;
      success.set(message);
    }
    console.log(`success response from Rust: ${JSON.stringify(res)}`);

    return res;
  } catch (err) {
    console.log(`error response from Rust: ${JSON.stringify(err)}`);
    error.set(err);
    return null;
  }
};

export const getError = () => {
  messenger({
    cmd: 'getError'
  });
};
