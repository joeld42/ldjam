//
// Bundle file
// Code changes will be overwritten
//

// title:  Numbers Station
// author: joeld42@gmail.com
// desc:   LD59 Game (theme: signal)
// script: wren

// Screen 240x136 center 120,68
// -------------------------------------------------------------------------

class RandomGenerator {
	construct new(seed) {
		_seed = seed == 0 ? 1 : seed  // Ensure non-zero seed
	}

	next() {
		_seed = _seed ^ (_seed << 13)
		_seed = _seed ^ (_seed >> 17)
		_seed = _seed ^ (_seed << 5)
		return _seed & 0x7FFFFFFF  // Keep positive
	}

	range(max) {
		return (next() % max).abs
	}

	range(min, max) {
		return min + range(max - min)
	}

	pick(list) {
		if (list.count == 0) return null
		return list[range(list.count)]
	}
}
var RNG = RandomGenerator.new( TIC.time() )

// -------------------------------------------------------------------------
// Asset in the "humint" sense not a game asset
class Asset {
	construct new( name, handler, pronouns, backstory ) {
		_name = name

		// If handler is null, it's an Agent.
		_handler = handler
		_pronouns = pronouns
		_backstory = backstory
	}

	// getter
	name { _name }
	handler { _handler }

	isAgent { _handler == null }

	toString { "<Asset (%(_name))>"}
}

var SHE_HER = [ "she", "her"]
var HE_HIM = [ "he", "him"]
var THEY_THEM = [ "they", "them"]

var ASSETS = [

	// Assets
	Asset.new( "Anais", null, SHE_HER,
		"Anais runs a flower shop in the old city. She sends coded messages to $AGENT by arranging the window display." ),
	Asset.new( "Oskar", null, HE_HIM,
		"Oksar is a plumber by trade. We can use his job to gain access to important homes and offices."),
	Asset.new( "Tessa", null, SHE_HER,
		"A firebrand revolutionary, Tessa is already drawing attention, but she is willing to take more risks than a typical asset." ),
	Asset.new( "Micah", null, HE_HIM, "todo: backstory" ),
]

var AGENTS = [
	Asset.new( "Peter", null, HE_HIM,
		"Oxford educated, Peter's charisma gives him an edge in recruiting assets."),
]



// -------------------------------------------------------------------------
class SpyGame {
	construct new() {
		_agents = {}
		_assets = {}

		// start with ONE random agent
	}

}

// // -------------------------------------------------------------------------
// class Mode_Transmitter {
// 	construct new() {}

// 	update(t) {
// 		EVENTS[0].activate( "hello")
//    	}

//    draw(t) {
// 		TIC.cls(10)
// 		TIC.print("TRANSMIT. ",
// 	  		30, 60,
// 			15, false,
// 			2, false
// 			)
//    }
// }

// // -------------------------------------------------------------------------
// class Mode_Intro {
// 	construct new() {}
// 	update(t) {

// 		if (TIC.btnp(4)) {
// 			Mode.transmitter()
// 		}
//    }

//    draw(t) {

// 	  // FIXME: vbank broken in wren binding??
// 	  //var b = TIC.vbank(1)

//       TIC.cls(13)

// 	  //var result = Math.sin(Math.pi / 2)
// 	  var p1x = (t * Num.pi/180).cos
// 	  var p1y = (t * Num.pi/180).sin

// 	  var p2x = ((t+120) * Num.pi/180).cos
// 	  var p2y = ((t+120) * Num.pi/180).sin

// 	  var p3x = ((t+240) * Num.pi/180).cos
// 	  var p3y = ((t+240) * Num.pi/180).sin

// 	  TIC.ttri( 120 + (p1x*30), 68 + (p1y*30),
// 	  	120 + (p2x*30), 68 + (p2y*30),
// 	  	120 + (p3x*30), 68 + (p3y*30),
// 		0, 0,
// 		32, 0,
// 		0, 32, 0, 0)

// 		TIC.spr( 33,
// 			120 + (p1x * 40),
// 			68  + (p1y * 30),
// 	  		0,1,0,0,2,2)
//       TIC.print("LUDUMDARE 59. ZZ2",
// 	  		30, 60,
// 			15, false,
// 			2, false
// 			)

//    }
// }

// // -------------------------------------------------------------------------
// class Mode {
//    static intro() {
//       //Timer.clear()
//       __mode = Mode_Intro.new()
//       System.gc()
//    }

//    static transmitter() {
//     //   Timer.clear()
//       __mode = Mode_Transmitter.new()
//       System.gc()
//    }

//    static update(i) {
//       __mode.update(i)
//    }

//    static draw(t) {
//       __mode.draw(t)
//    }
// }

// -------------------------------------------------------------------------
class GameMode {
	static mainmenu { 0 }
	static office { 1 }
	static corkboard { 2 }
	static citymap { 3 }
}

class Game is TIC{

	construct new(){
		_t=0
		_mode = GameMode.mainmenu
		//_mode = GameMode.office
		_menu_nums = (0..99).map { |i| RNG.range(100) }.toList
		_menuoffs = 10
		_starter = 1000
		_office_title_w = 100
		_office_desc_w = 130
	}

	mainmenu_tick() {

		// Draw a numbers pattern
		TIC.cls(15)
		for (row in 0..20) {
			for (col in 0..30) {
				var offs = 0
				if (row % 2==0) {
					offs = 12
				}

				var ndx = (row * 20 + col) % _menu_nums.count
				var num = _menu_nums[ndx]

				if (num > _starter) {
					continue
				}

				var numstr = num < 10 ? "0%(num)" : "%(num)"
				TIC.print( numstr,col*30+offs - _menuoffs,
					row*13 - _menuoffs/6, 14, true, 2 )
			}
		}

		// change the numbers
		var ndx = RNG.range(_menu_nums.count)

		_menu_nums[ndx] = RNG.range(100)

		if (_t % 3==0 ){
			_menuoffs = _menuoffs + 1
		}

		if (_menuoffs > 400) {
			_menuoffs = 10
		}

		TIC.print( "Numbers\nStation", 22, 22, 15, false, 3 )
		TIC.print( "Numbers\nStation", 20, 20, 8, false, 3 )
		TIC.print( "A Game for LD59 by joeld42", 20, 58, 2, false, 1, true )

		var s = (100 - _starter).max(0)
		TIC.print( s, 10, 10, 1 )
		TIC.print( "> Start Game", 120 - 36, 100+s, 8, false, 1 )

		if ((TIC.btnp(4) || TIC.btnp(5)) && _starter > 100 ) {
			_starter = 99
		}

		if (_starter < 100) {
			_starter = _starter -1
			if (_starter <= 0) {
				_mode = GameMode.office
			}
		}
	}

	spriteHover(mx, my, x, y, w, h) {
    	return mx >= x && mx < x + (w * 8) && my >= y && my < y + (h * 8)
	}

	sprDrawHover( mouse, id, x, y, colorkey, scale, flip, rotate, w, h) {

		// Check if hovering (w and h default to 1 if not provided)
		var sprite_w = w != null ? w : 1
		var sprite_h = h != null ? h : 1

		// Get mouse position
		var mx = mouse[0]
		var my = mouse[1]
		var sel = spriteHover(mx, my, x, y, sprite_w, sprite_h)

		// If selected draw a circle behind sprite
		if (sel) {
			TIC.circ( x + (w*4), y + (w*4), w*6, 10 )
			TIC.circ( x + (w*4), y + (w*4), w*5, 9 )
		}

		// Draw the sprite
		if (sel) {
			y = y - 3
		}
		TIC.spr(id, x, y, colorkey, scale, flip, rotate, w, h)



		return sel
	}

	office_tick() {

		TIC.cls(3)
		TIC.rect( 0, 136/2-30, 240, 60, 2  )

		var mouse = TIC.mouse()

		// Corkboard
		var title = ""
		var desc = ""
		if (sprDrawHover( mouse, 32, 30, 55, 0,
						1, 0, 0, 4, 4 )) {
			title = "Corkboard"
			desc = "Review assets and their connections."
			if (mouse[2]) {
				_mode = GameMode.citymap
			}
		}

		// Telephone
		if (sprDrawHover( mouse, 36, 90, 55, 0,
			1, 0, 0, 4, 4 )) {
				title = "Embassy Phone"
				desc = "Coordinate with field agents."
				if (mouse[2]) {
					_mode = GameMode.citymap
				}
			}

		// Transmitter
		if (sprDrawHover( mouse, 40, 160, 55, 0,
			1, 0, 0, 4, 4 )) {
				title = "Transmitter"
				desc = "Send coded messages to assets."
				if (mouse[2]) {
					_mode = GameMode.citymap
				}
			}

		if (title != "") {
			_office_title_w = TIC.print( title,
			 (240 - _office_title_w) / 2, 30, 8 )

			 _office_desc_w = TIC.print( desc,
			 (240 - _office_desc_w) / 2, 110, 1 )
		}
	}

	citymap_tick() {

		TIC.cls(15)
		TIC.map( 0, 0 )

	}

	BOOT() {
	}

	TIC() {
		if (_mode == GameMode.mainmenu) {
			mainmenu_tick()
		} else if (_mode == GameMode.office) {
			office_tick()
		} else if (_mode == GameMode.citymap) {
			citymap_tick()
		}
		// if(TIC.btn(0)){
		// 	_y=_y-1
		// }
		// if(TIC.btn(1)){
		// 	_y=_y+1
		// }
		// if(TIC.btn(2)){
		// 	_x=_x+1
		// }
		// if(TIC.btn(3)){
		// 	_x=_x-1
		// }

		// TIC.cls(13)
		// TIC.spr(33,_x,_y,14,1,0,0,2,2)
		// TIC.print("HELLO WORLD!",_x,84)

		//TIC.trace( "frame %(_t)")

		_t=_t+1
	}
}