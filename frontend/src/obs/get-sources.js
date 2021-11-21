import { connection } from '.';

export default async () => {
  try {
    return await connection.send('GetSceneList');
  } catch (exc) {
    console.error(exc);
  }
};
