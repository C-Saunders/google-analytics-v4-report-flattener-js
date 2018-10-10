var addon = require('../native');

module.exports = {
  toDelimited,
  toFlatJson,
};

function toDelimited(data, delimiter) {
  return addon.toDelimited(getAsString(data), delimiter);
}

function toFlatJson(data) {
  return JSON.parse(addon.toFlatJsonString(getAsString(data)));
}

function getAsString(data) {
  if (typeof data !== 'string') {
    return JSON.stringify(data);
  }
  return data;
}
