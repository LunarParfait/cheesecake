
const UglifyJS = require("uglify-js");
const UglifyCSS = require("uglifycss");
const fs = require("fs");
const path = require("path");

/**
  *
  * @returns {string[]}
  */
function* readAllFiles(dir) {
  const files = fs.readdirSync(dir, { withFileTypes: true });

  for (const file of files) {
    if (file.isDirectory()) {
      yield* readAllFiles(path.join(dir, file.name));
    } else {
      yield path.join(dir, file.name);
    }
  }
}

const dirin = "public"
const dirout = "dist"

for (const file of readAllFiles(dirin)) {
  console.log(`minifying ${file}`);
  const ext = file.split(".").slice(-1)[0];

  let data = fs.readFileSync(file, {encoding: "utf8", flag: "r"});
  let out;

  switch (ext) {
    case "css":
      out = UglifyCSS.processString(data);
      break;
    case "js":
      out = UglifyJS.minify(data);
      break;
    default:
      out = data;
  }

  const filename = file.split(dirin).slice(-1)[0];

  fs.writeFileSync(dirout + filename, out);
}
