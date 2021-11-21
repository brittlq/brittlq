import { connection } from '.';

/**
 * @param {String} sourceName
 */
export default async (sourceName) => {
  try {
    return await connection.send('SetSourceSettings', {
      sourceName,
      sourceSettings: {},
    });
  } catch (exc) {
    console.error(exc);
  }
};
