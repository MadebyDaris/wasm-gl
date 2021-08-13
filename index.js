// import * as m from "./pkg";

const rust = import('./pkg/index');
const canvas = document.getElementById('page');
const gl = canvas.getContext('webgl', { antialias: true });

rust.then(m => {
    if (!gl) {  
        alert( 'webgl is broken' );
        return; 
    }
    
    const throt = 1000.0 / 30.0;
    const myclient = new m.MyClient();
    
    const initTime = Date.now();
    let last_draw_time = -1;

    function render() {
        window.requestAnimationFrame(render);
        const currTime = Date.now();
        
        if (currTime >= last_draw_time + throt ) {
            last_draw_time = currTime;

            if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
                canvas.height = window.innerHeight;
                canvas.clientHeight  = window.innerHeight;
                canvas.style.height = window.innerHeight;

                canvas.width = window.innerWidth;   
                canvas.clientWidth  = window.innerWidth;
                canvas.style.width = window.innerWidth;

                gl.viewport(0, 0, window.innerWidth, window.innerHeight);
            }

            let timePassed = currTime - initTime;
            myclient.update(timePassed, window.innerHeight, window.innerWidth);
            myclient.render();
        }
    }
    render();
}).catch(console.error);