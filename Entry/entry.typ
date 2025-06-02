#import "@preview/cetz:0.3.4"
#set page(
	height:auto,
	width:auto,
	margin:5pt,
)
#set text(
	font:"Noto Sans",
	size:20pt,
)
#let ch(dir:int,x:float,y:float,body)={
	cetz.draw.content(
		(2*x+y,1.73205*y),
		angle:dir*60deg-90deg
	)[
		#body
	]
}
//Noisy Shadows in the Horizon
#cetz.canvas({
	ch(dir:5,x:-1,y:1)[N]
	ch(dir:0,x:0,y:0)[O]
	ch(dir:2,x:1,y:0)[I]
	ch(dir:1,x:0,y:1)[S]
	ch(dir:2,x:0,y:2)[Y]
	ch(dir:2,x:-1,y:3)[S]
	ch(dir:4,x:-2,y:4)[H]
	ch(dir:5,x:-2,y:3)[A]
	ch(dir:3,x:-1,y:2)[D]
	ch(dir:4,x:-2,y:2)[O]
	ch(dir:4,x:-2,y:1)[W]
	ch(dir:0,x:-2,y:0)[S]
	ch(dir:5,x:-1,y:0)[I]
	ch(dir:3,x:0,y:-1)[N]
	ch(dir:5,x:-1,y:-1)[T]
	ch(dir:0,x:0,y:-2)[H]
	ch(dir:4,x:1,y:-2)[E]
	ch(dir:0,x:1,y:-3)[H]
	ch(dir:1,x:2,y:-3)[O]
	ch(dir:2,x:2,y:-2)[R]
	ch(dir:0,x:1,y:-1)[I]
	ch(dir:1,x:2,y:-1)[Z]
	ch(dir:2,x:2,y:0)[O]
	ch(dir:2,x:1,y:1)[N]
})
