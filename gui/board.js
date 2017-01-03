var Board = new Vue({
  el: '#gomoku',
  data: {
    squares: [],
    thinking: false,
    ready: false,
    status: ''
  }
})
AI = true
'.'.repeat(225).split('').map(() => {
  Board.squares.push({
    active: false,
    value: '',
    black: false,
    white: false,
    evaluated: false
  })
})

Board.activate = function (sq, me) {
  if (sq.active) return
  sq.active = true
  sq[(Board.$data.thinking?'black':'white')] = true 
  Board.$data.thinking = !Board.$data.thinking
  if (AI && !me) {
    engine.stdin.write(Board.$data.squares.map((i) => (i.black?1:(i.white?2:0))).join(',')+'\n')
  }
}


Board.clear = ()  => {
  Board.$data.squares.forEach(i => {
    i.active = false
    i.white = false
    i.black = false
    i.evaluated = false
    i.value = ''
  })
}
Board.toString = function () {

}
var to = {}
