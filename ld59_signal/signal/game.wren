import "luxe: world" for World, Entity, Transform, Sprite, Values, Tags, Camera
import "luxe: world/scene" for Scene

import "luxe: color" for Color
import "luxe: draw" for Draw, PathStyle
import "luxe: input" for Input, Key
import "luxe: assets" for Assets
import "luxe: asset" for Asset
import "luxe: math" for Math
import "luxe: io" for IO

import "debug: debug"

import "luxe: ui" for UIButton, Control, UILabel, UISlider, UIPanel, UIImage, UIText, UIEvent
import "luxe: world" for UI, UIBehave, UIContain
import "luxe: game" for Frame

import "luxe.project/asset" for CardInfo
import "outline/ready" for Ready

import "mainmenu" for MainMenu
import "office" for Office
import "citygame" for CityGame

class Game is Ready {



  construct ready() {

    super("ready! %(width) x %(height) @ %(scale)x")

    // Load card data
    var db = Assets.db_default()
    var cards = Assets.list_ids( db, "type/card.asset" )
    Log.print("Cards is %(cards)" )

    //var card = Asset.get.card("cards/testcard")
    for (card_id in cards) {
      var card = Asset.get.card(card_id)
      Log.print("Card is %(card_id) ------------")
      Log.print("%(card.title) --  %(card.text)" )
      Log.print("Outcomes : %(card.outcomes.count)")
      for ( oc in card.outcomes ) {
        // var oc = card.outcomes[i]
        Log.print("Outcome: %(oc.prompt)")
        var cmds = oc.cmds
        for (cmd in cmds.split(";")) {
          Log.print("CMD: %(cmd)")
        }
      }
    }

    _mode = MainMenu.new( world, camera, Fn.new {
        Log.print("Hello from CB Start!")
        Frame.end {
          _mode.teardown()
          //_mode = Office.new( world, camera )
          _mode = CityGame.new( world, camera )
        }

      })

  } //ready

  tick(delta: Num) {

    // Transform.set_pos(character, mouse.x, mouse.y)
    _mode.tick( delta, mouse )

    if(Input.key_state_released(Key.escape)) {
      IO.shutdown()
    }

  } //tick

} //Game
