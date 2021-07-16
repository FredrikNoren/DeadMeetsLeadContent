const fs = require('fs');
const path = require('path');
const { exec } = require('child_process');

async function handle(dir) {
    // ~/github/assimp/bin/assimp export Altar1.x Altar1.glb -fglb2
    let files = fs.readdirSync(dir);
    for (let i = 0; i < files.length; i++) {
        const fullname = path.join(dir, files[i]);
        const isDir = fs.lstatSync(fullname).isDirectory();
        if (isDir) {
            handle(fullname);
        } else {
            if (fullname.endsWith(".x")) {
                const newname = fullname.slice(0, fullname.length - 2) + ".glb";
                console.log(newname);
                await new Promise((resolve) => {
                    exec(`~/github/assimp/bin/assimp export ${fullname} ${newname} -fglb2`, resolve);
                });
            }
        }
    }
}
handle(".").then(x => { console.log('done') });
