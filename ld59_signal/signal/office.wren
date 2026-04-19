import "luxe: world" for World, Entity, Transform, Sprite, Values, Tags, Camera, Text
import "luxe: world/scene" for Scene

import "luxe: color" for Color
import "luxe: draw" for Draw, PathStyle
import "luxe: render" for TextAlign
import "luxe: input" for Input, Key
import "luxe: assets" for Assets
import "luxe: asset" for Asset
import "luxe: math" for Math
import "luxe: io" for IO

import "luxe: ui" for UIButton, Control, UILabel, UISlider, UIPanel, UIImage, UIText, UIEvent
import "luxe: world" for UI, UIBehave, UIContain
import "random" for Random

// Card size 120x180

class Office {

    // dbg
    swipecard { _swipecard }

    construct new( world, camera ) {
         _officescene = Scene.create(world, Asset.scene("scene/office"))

        _rng = Random.new()
        _font = Asset.font("luxe: font/lato")

        _cardinfo = {} // "Title" : Card
        _deck = [ "cards/starter" ] // Current deck (ids)
        _card = null

        // The swipe card entity
        _swipecard = Entity.create(world, "card")

        _draw = Draw.create(World.render_set(world))


        Transform.create(_swipecard, 160, 120, 5 )
        Sprite.create(_swipecard, Assets.image("image/cardframe"), 120, 180)

        var cardTitle = Entity.create( world, "title")
        Transform.create( cardTitle, 85, 50, 50)
        Text.create( cardTitle, Assets.material("luxe: material/font"), 36,
            Asset.font("luxe: font/lato"),
                [1,1,1,1])
        Text.set_text( cardTitle, "Card Title")
        //Sprite.create(cardTitle, Assets.image("image/cardframe"), 0, 70 )
         Transform.link( cardTitle, _swipecard )

    }

    tick(delta: Num, mouse : Float2 ) {


        Transform.set_pos( _swipecard, mouse.x, mouse.y)

        // Draw a card if we don't have one
        if (_card == null) {

            if (_deck.count == 0 ) {
                // deck empty you lose
                Draw.text( _draw, 50, 32, 0, "You Lose", 12, Color.red)
                Draw.commit(_draw)

            }

            _rng.shuffle( _deck )
            _card = _deck.removeAt( -1 )
        }




        // TMP drawing
        var x = 320 / 2
        var y = 240 / 2
        var depth = 100
        var radius = 16
        var smooth = 6

        Draw.circle(_draw, x, y, depth, radius+4, smooth, Color.white)
        Draw.circle(_draw, x, y, depth, radius, smooth, Color.pink)

        var size = 12

        // Draw.text( _draw, 20, 15, depth, _card, size, Color.white )

        Draw.text( _draw, 20, 15, depth,
                _card, size, _font, Color.white, TextAlign.left, TextAlign.bottom )


        Draw.commit(_draw)
    }

}
