import "luxe: world" for World, Entity, Transform, Sprite, Values, Tags, Camera
import "luxe: world/scene" for Scene

import "luxe: color" for Color
import "luxe: draw" for Draw, PathStyle
import "luxe: input" for Input, Key
import "luxe: assets" for Assets
import "luxe: asset" for Asset
import "luxe: math" for Math
import "luxe: io" for IO

import "luxe: ui" for UIButton, Control, UILabel, UISlider, UIPanel, UIImage, UIText, UIEvent
import "luxe: world" for UI, UIBehave, UIContain


class MainMenu {


    construct new( world, camera, cb_start ) {

        _menuscene = Scene.create(world, Asset.scene("scene/mainmenu"))

        // character = Entity.create(world)
        // Transform.create(character)
        // Sprite.create(character, Assets.image("luxe: image/luxe.char"), 58, 136)
        // Sprite.set_pixelated(character, true)
        var canvas = Entity.create(world, "ui")
        _ui_canvas = canvas
        Log.print("Canvas is %(_ui_canvas)" )

        var x = 0
        var y = 0
        var w = 320 //these are in world units
        var h = 240
        var depth = 0
        UI.create(canvas, x, y, w, h, depth, camera)

        var bg = Control.create(canvas)
        Control.set_behave(bg, UIBehave.fill)
        // Control.set_contain(bg, UIContain.middle | UIContain.column )
        // UIPanel.set_color(bg, Color.clear )

        var btn_x = (320 - 128) /2
        var btn_y = 120
        var btn_start = UIButton.create(canvas)
        UIButton.set_text(btn_start, "Start Game")
        // Control.set_behave(btn_start, UIBehave.center)
        Control.set_pos( btn_start, btn_x, btn_y )
        Control.set_size(btn_start, 128, 32)
        Control.child_add(bg, btn_start)
        btn_y = btn_y + 36

        Control.set_events(btn_start) {|event|
            if(event.type == UIEvent.release) {
            Log.print("The button was clicked! x %(event.x) y %(event.y)")
            cb_start.call()
        }
    }

        var btn_quit = UIButton.create(canvas)
        UIButton.set_text(btn_quit, "Quit")
        //Control.set_behave(btn_quit, UIBehave.center)
        Control.set_pos( btn_quit, btn_x, btn_y )
        Control.set_size(btn_quit, 128, 32)
        Control.child_add(bg, btn_quit )

        UI.commit( canvas )

    }

    teardown() {
        Log.print("Teardown: Canvas is %(_ui_canvas)" )
        Log.print("Teardown: Menu scene %(_menuscene)" )
        Entity.destroy( _ui_canvas )
        Entity.destroy( _menuscene )

        Log.print("Teardown: done" )
    }

    tick(delta: Num, mouse : Float2 ) {}

}