let pkg;

import("./pkg")
  .then(m => { pkg = m; })
  .catch(e => console.error(e));


document.querySelector("#input").addEventListener("change", onChange);

function onChange(e) {
  [...e.target.files].forEach(file => {
    let reader = new FileReader();

    reader.onload = onFileLoad;

    reader.readAsArrayBuffer(file);
  });
}

function onFileLoad(e) {
  let el = document.createElement("div");
  el.className = "picture";

  el.innerHTML = `<img>`;
  let img = el.querySelector("img");

  document.querySelector("#pictures").insertBefore(el, null);

  img.width = 100;
  img.src = URL.createObjectURL(new Blob([e.target.result]));

  try {
    pkg.get_exif(new Uint8Array(e.target.result), (tag, value) => {
      img.title += `${tag}: ${value}; `;
    });
    if (img.title) {
      img.className = "success";
    } else {
      img.className = "no-metadata";
    }
  } catch (e) {
    img.title = "(Error fetching metadata)";
    img.className = "error";
  }
}
