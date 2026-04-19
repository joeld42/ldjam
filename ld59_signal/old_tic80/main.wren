// title:  Numbers Station
// author: joeld42@gmail.com
// desc:   LD59 Game (theme: signal)
// script: wren

// Screen 240x136 center 120,68
// -------------------------------------------------------------------------

class RandomGenerator {
	construct new( seed ) {
		_seed = seed
	}

	next() {
		_seed = (_seed * 1103515245 + 12345) % (2.pow(31))
		return _seed
	}
	nextInt() {
		return next().round
	}

	range( max ) {
		return nextInt() % max
	}

	range( min, max ) {
		return min + (nextInt() % (max-min))
	}

	pick( list ) {
		if (list.count == 0) return null
		var ndx = nextInt() % list.count
		return list[ndx]
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
class Event {
	construct new( title, text ) {
		_title = title
		_text = text
		_action = null
	}

	construct new( title, text, action ) {
		_title = title
		_text = text
		_action = action
	}

	title { _title }
	text { _text }

	activate( params ) {
		if (_action != null) {
			_action.call( params )
		}
	}

	toString { "<Event (%(_title))>"}
}

var EVENTS = [
	Event.new( "Paranoia Strikes Deep",
		"$ASSET reports that $SHE feels like $SHE is being watched.",
		Fn.new { |asset|
			TIC.trace("In ACTION for event")
		}
	)
]

// -------------------------------------------------------------------------
class SpyGame {
	construct new() {
		_agents = {}
		_assets = {}

		// start with ONE random agent
	}

}

// -------------------------------------------------------------------------
class Mode_Transmitter {
	construct new() {}

	update(t) {
		EVENTS[0].activate( "hello")
   	}

   draw(t) {
		TIC.cls(10)
		TIC.print("TRANSMIT. ",
	  		30, 60,
			15, false,
			2, false
			)
   }
}

// -------------------------------------------------------------------------
class Mode_Intro {
	construct new() {}
	update(t) {

		if (TIC.btnp(4)) {
			Mode.transmitter()
		}
   }

   draw(t) {

	  // FIXME: vbank broken in wren binding??
	  //var b = TIC.vbank(1)

      TIC.cls(13)

	  //var result = Math.sin(Math.pi / 2)
	  var p1x = (t * Num.pi/180).cos
	  var p1y = (t * Num.pi/180).sin

	  var p2x = ((t+120) * Num.pi/180).cos
	  var p2y = ((t+120) * Num.pi/180).sin

	  var p3x = ((t+240) * Num.pi/180).cos
	  var p3y = ((t+240) * Num.pi/180).sin

	  TIC.ttri( 120 + (p1x*30), 68 + (p1y*30),
	  	120 + (p2x*30), 68 + (p2y*30),
	  	120 + (p3x*30), 68 + (p3y*30),
		0, 0,
		32, 0,
		0, 32, 0, 0)

		TIC.spr( 33,
			120 + (p1x * 40),
			68  + (p1y * 30),
	  		0,1,0,0,2,2)
      TIC.print("LUDUMDARE 59. ZZ2",
	  		30, 60,
			15, false,
			2, false
			)

   }
}

// -------------------------------------------------------------------------
class Mode {
   static intro() {
      //Timer.clear()
      __mode = Mode_Intro.new()
      System.gc()
   }

   static transmitter() {
    //   Timer.clear()
      __mode = Mode_Transmitter.new()
      System.gc()
   }

   static update(i) {
      __mode.update(i)
   }

   static draw(t) {
      __mode.draw(t)
   }
}
// -------------------------------------------------------------------------
class Game is TIC{

	construct new(){
		_t=0
		Mode.intro()
	}

	BOOT() {
		// In your transmitter mode, test these:
		TIC.trace("Testing random functions...")

		// Test 1
		var r1 = RNG.next()
		TIC.trace("TIC.math.random works: %(r1)")

		var r2 = RNG.nextInt()
		TIC.trace("TIC.math.random works: %(r2)")

		// // Test 2
		// try
		// {
		// 	var r2 = math.random(10)
		// 	TIC.trace("math.random works: %(r2)")
		// } catch
		// {
		// 	TIC.trace("math.random not available")
		// }

		// // Test 3
		// try {
		// 	var r3 = random(10)
		// 	TIC.trace("random works: %(r3)")
		// } catch {
		// 	TIC.trace("random not available")
		// }
	}

	TIC(){
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
		Mode.update(_t)
		Mode.draw(_t)
		//TIC.trace( "frame %(_t)")

		_t=_t+1
	}
}