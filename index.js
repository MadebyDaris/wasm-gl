import('./pkg/').then((lib) =>{
    console.log(`2 + 2 = ${lib.add(2, 2)}`)
})