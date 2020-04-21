import { error, success } from '../stores';

export const messenger = async (payload, informSuccess = false) => {
  try {
    const res = await window.tauri.promisified(payload);

    if (informSuccess) {
      success.set(res);
    }
    console.log(`success response from Rust: ${JSON.stringify(res)}`);
    return res;
  } catch (e) {
    const err = JSON.parse(e);
    console.log(`error response from Rust. Operation: ${err.operation}`);
    console.log(`error response from Rust. err: ${err.err}`);
    error.set(err);
    return null;
  }
};

export const getError = () => {
  messenger({
    cmd: 'getError',
  });
};

export const query = () => {
  window.tauri.invoke({
    cmd: 'testQuery',
  });
};
