const { minify } = require("html-minifier-terser");
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

const pubin = "resources/static"
const pubout = "dist/static"
const templin = "resources/templates"
const templout = "dist/templates"

const jinjaTagPattern = /({{.*?}}|{%.*?%}|{#.*?#})/gs;

const minifyOpts = {
  collapseWhitespace: true,
  removeComments: true,
  removeRedundantAttributes: true,
  removeEmptyAttributes: true,
  collapseWhitespace: true,
  conservativeCollapse: true,
  useShortDoctype: true,
  minifyCSS: true,
  minifyJS: true,
}

/**
  * @param content {string}
  * @returns {Promise<string>}
  */
async function minifyTemplate(content) {
  const split = content.split(jinjaTagPattern)
  let result = "";
  for (part of split) {
    result += jinjaTagPattern.test(part) ? part : await minify(part, minifyOpts)
  }

  return result
}

async function main() {
  for (const file of readAllFiles(pubin)) {
    console.log(`minifying asset ${file}`);
    const ext = file.split(".").slice(-1)[0];

    let data = fs.readFileSync(file, { encoding: "utf8", flag: "r" });
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

    const filename = file.split(pubin).slice(-1)[0];

    fs.writeFileSync(pubout + filename, out);
  }

  for (const file of readAllFiles(templin)) {
    console.log(`minifying template ${file}`);

    let data = fs.readFileSync(file, { encoding: "utf8", flag: "r" });
    let out = await minifyTemplate(data);
    const filename = file.split(templin).slice(-1)[0];

    fs.writeFileSync(templout + filename, out);
  }
}


main()
