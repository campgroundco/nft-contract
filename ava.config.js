// require("util").inspect.defaultOptions.depth = 5; // Increase AVA's printing depth

module.exports = {
    timeout: "300000",
    files: ["e2e/*.ava.ts"],
    extensions: ["ts"],
    require: ["ts-node/register"],
};
