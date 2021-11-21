import { connection } from '.';

export default async () => {
  try {
    return await connection.send('GetSourceSettings');
  } catch (exc) {
    console.error(exc);
  }
};
