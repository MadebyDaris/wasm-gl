import * as m from "./pkg"; 

const rust = import('./pkg/crb');
const canvas = document.getElementById('rustCanvas');
const gl = canvas.getContext('webgl', { antialising: true });

rust.then(m => {
    if (!gl) { alert( "webgl is broken" ) }

    gl.enable(gl.BLEND);
    gl.blendfunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);
    
    const throt = 1000 / 30;
    const client = new m.client();
    
    var last_draw_time = -1;
    var initTime = date.now

    function render() {
        window.requestAnimationFrame(render);
        const currTime = Date.now();
        
         if (currTime >=  last_draw_time ) {
            last_draw_time = currTime

            if (window.width != canvas.width || window.height != canvas.height) {
                canvas.height = window.height;
                canvas.clientHeight  = window.innerHeight;
                canvas.style.height = window.innerHeight;

                canvas.width = window.width
                canvas.clientWidth  = window.innerWidth;
                canvas.style.width = window.innerWidth;

                gl.viewport(0, 0, window.innerWidth, window.innerHeight);
            }

            let timePassed = currTime - initTime;
            client.update(timePassed, window.innerHeight, window.innerWidth);
            clint.render();
        }
    }
    render()
})