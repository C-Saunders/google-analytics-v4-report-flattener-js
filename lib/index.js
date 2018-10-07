var addon = require('../native');

module.exports = {
  toDelimited,
  toFlatJsonString,
};

function toDelimited(data, delimiter) {
  return addon.toDelimited(getAsString(data), delimiter);
}

function toFlatJsonString(data) {
  return JSON.parse(addon.toFlatJsonString(getAsString(data)));
}

function getAsString(data) {
  if (typeof data !== 'string') {
    return JSON.stringify(data);
  }
  return data;
}
