// Start the Rust Engine
var spawn = require('child_process').spawn;
var engine = spawn('../target/release/sandbox.exe')

engine.stderr.on('data', function (data) {
  console.log(data + '')
})
engine.stdout.on('data', function (data) {
  data += ''
  data.split('\n').filter(i => i).forEach(parse)
});


function parse(data) { 
  var things = data.split(':');
  if (things.length < 2) {
    console.log(data)
    return
  }

  switch (things[0]) {
    case 'COMMENT':
      console.log(data)
      Board.$data.status = things[1]
    break
    case 'EVENT':
      Board.$data.ready = true
    break
    case 'TIME':
    console.log(data)
    console.log('Node Evaluation to: ', things[1])
    break
    case 'MOVES':
      var arr = things[1].replace(/\(/g,'[').replace(/\)/g,']')
      console.log(arr)
      Board.$data.squares.forEach((sq) => {
        sq.value = ''
      })
      JSON.parse(arr).filter(tup => tup[1] != 0).forEach((i) => {
        Board.$data.squares[i[0]].value = i[1]
      })
      setTimeout(() => {
        Board.activate(Board.$data.squares[JSON.parse(arr)[0][0]], true)
      },10)
    break
    case 'DONE':
      console.log(data)
      var arr = JSON.parse(things[1].replace(/\(/g,'[').replace(/\)/g,']'))
      Board.$data.squares[arr[0]].evaluated = true
    break
    case 'VALUES':
      Board.$data.squares.forEach((sq) => {
        sq.value = ''
      })
      var arr = things[1].replace(/\(/g,'[').replace(/\)/g,']')
      JSON.parse(arr).filter(tup => tup[1] != 0).forEach((i) => {
        Board.$data.squares[i[0]].value = i[1]
        Board.$data.squares[i[0]].evaluated = false        
      })
      console.log(arr)
    break
    case 'VALUE':
      console.log('Board value is ',things[1])
    break
  }



}