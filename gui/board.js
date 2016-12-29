var Board = new Vue({
  el: '#gomoku',
  data: {
    squares: [],
    thinking: false,
    ready: false,
    status: ''
  }
})

'.'.repeat(225).split('').map(() => {
  Board.squares.push({
    active: false,
    value: ''
  })
})

Board.activate = function (sq, me) {
  if (sq.active) return
  sq.active = true
  sq[(Board.$data.thinking?'white':'black')] = true 
  Board.$data.thinking = !Board.$data.thinking
  if (!me) {
    engine.stdin.write(Board.$data.squares.map((i) => (i.black?2:(i.white?1:0))).join(',')+'\n')
  }
}

Board.clear = ()  => {
  Board.$data.squares.forEach(i => !(i.active = false)&&(delete i.white||delete i.black)&&(i.value = ''))
}
Board.toString = function () {

}
var to = {}
