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

class CityGame {

    construct new( world, camera ) {
            _cityscene = Scene.create(world, Asset.scene("scene/city"))
            _camera = camera
    }

    tick(delta: Num, mouse : Float2 ) {

        var offsx = 320 /2
        var offsy = 240 /2
        Transform.set_pos( _camera, -offsx, -offsy)

    }
}