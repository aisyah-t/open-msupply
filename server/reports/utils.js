const cleanUpNodes = (nodes) => {
  let cleanNodes = [];
  nodes.forEach((node) => {
    if (Object.keys(node).length != 0) {
      cleanNodes.push(cleanUpObject(node));
    }
  });
  return cleanNodes;
};

const cleanUpObject = (node) => {
  let newNode = {};
  // remove empty keys which will fail to parse
  Object.keys(node).forEach(function (key) {
    if (node[key] !== "" && node[key] !== undefined && node[key] !== null) {
      if (typeof node[key] === "object") {
        // recursively remove empty strings or undefined from graphql query
        newNode[key] = cleanUpObject(node[key]);
      } else {
        newNode[key] = node[key];
      }
    }
  });
  return newNode;
};

module.exports = {
  cleanUpObject,
  cleanUpNodes,
};
