import "luxe: world" for World, Camera, Entity, Transform, Sprite
import "luxe: render" for Render, Material, Image
import "luxe: game" for Frame, Ready as Main
import "luxe: settings" for Settings
import "luxe: input" for Input
import "luxe: color" for Color
import "luxe: id" for ID
import "luxe: io" for IO

class Ready is Main {

  var world = World.create("game")
  var ui = World.create("ui")
  var display = World.create("display")

  var camera = Entity.create(world, "camera")
  var ui_camera = Entity.create(ui, "camera")
  var display_camera = Entity.create(display, "camera")

  var color : Color = Settings.get("game.world.color", [1,1,1,1])
  var border : Color = Settings.get("game.world.border", [0,0,0,1])

  width : Num { Render.window_w() }
  height : Num { Render.window_h() }
  scale : Num { Render.drawable_ratio() }

  world_width : Num { Settings.get("game.world.width", Render.window_w()) }
  world_height : Num { Settings.get("game.world.height", Render.window_h()) }
  game_region : Float4 { 

    var dest_size_x = width
    var dest_size_y = height
    var dest_size_half_x = (dest_size_x * 0.5).floor
    var dest_size_half_y = (dest_size_y * 0.5).floor
    var region = [0, 0, dest_size_x, dest_size_y]

    //so here we can check whether the input is
    //portrait or landscape in relation to the output size
    //if < 1, it's portrait. if >= 1, landscape
    var ratio = world_width / world_height
    var portrait = ratio < 1.0
    if(portrait) {
      region.z = (dest_size_y * ratio).floor
      region.x = (dest_size_half_x - (region.z * 0.5)).floor
      if(region.z > dest_size_x) {
        region.x = 0.0
        region.z = dest_size_x
        region.w = (dest_size_x / ratio).floor
        region.y = (dest_size_half_y - (region.w * 0.5)).floor
      }
    } else {
      region.w = (dest_size_x / ratio).floor
      region.y = (dest_size_half_y - (region.w * 0.5)).floor
      if(region.w >= dest_size_y) {
        region.y = 0.0
        region.w = dest_size_y
        region.z = (dest_size_y * ratio).floor
        region.x = (dest_size_half_x - (region.z * 0.5)).floor
      }
    }

    return region
  }

  mouse : Float2 {

    var mx = Input.mouse_x()
    var my = Input.mouse_y()
    var region = game_region
    var ratio_x = region.z / width
    var ratio_y = region.w / height

    mx = (mx - region.x) / ratio_x
    my = (my - region.y) / ratio_y

    my = my.floor
    mx = mx.floor

    var world_mouse = Camera.screen_point_to_world(camera, mx, my)
        world_mouse.x = world_mouse.x.round
        world_mouse.y = world_mouse.y.round
    return world_mouse
  }
  
  construct ready(message: String) {

    super(message)

    Log.print("pixel / outline / world color %(color)")
    Log.print("pixel / outline / world border color %(border)")
    Log.print("pixel / outline / world width %(world_width)")
    Log.print("pixel / outline / world height %(world_height)")
    Log.print("pixel / outline / window width %(width)")
    Log.print("pixel / outline / window height %(height)")

    _on_resize = {}

    //create cameras

      Transform.create(camera)
      Camera.create(camera)
      Camera.set_default(world, camera)
      Camera.set2D(camera, 0, 0, world_width, world_height, -100, 100)

      Transform.create(ui_camera)
      Camera.create(ui_camera)
      Camera.set_default(ui, ui_camera)

      Transform.create(display_camera)
      Camera.create(display_camera)
      Camera.set_default(display, display_camera)
      
      var region = game_region
      _display_sprite = Entity.create(display, "app.display")
      _display_mat = Material.create("luxe: material_basis/sprite_pixelated_AA")
      Material.set_input(_display_mat, "sprite.image", Image.get_resource("scene"))
      Sprite.create_with(_display_sprite, _display_mat, region.z, region.w)
      Sprite.set_origin(_display_sprite, 0, 0)
      Sprite.set_flip_v(_display_sprite, true)
      Transform.create(_display_sprite)
      Transform.set_pos(_display_sprite, region.x, region.y)

    //resize cameras on size changed

      IO.on("engine.runtime.window.size_changed") {|type, data|
        var region = game_region
        Log.print("outline / resized / %(Render.window_w())x%(Render.window_h()) region %(game_region)")
        Camera.set2D(camera, 0, 0, world_width, world_height, -100, 100)
        Camera.set2D(ui_camera, 0, 0, Render.window_w(), Render.window_h(), -100, 100)
        Camera.set2D(display_camera, 0, 0, Render.window_w(), Render.window_h(), -100, 100)

        Transform.set_pos(_display_sprite, region.x, region.y)
        Sprite.set_size(_display_sprite, region.z, region.w)

        _on_resize.each {|fn|
          fn.call(Render.window_w(), Render.window_h())
        }
      }

    //update our worlds

      Frame.on(Frame.sim) {|delta|
        World.tick(world, delta)
        World.tick(display, delta)
        World.tick(ui, delta)
      }

    //render our worlds

      Frame.on(Frame.visual) {|delta|
        World.render(world, camera, "game", { "clear_color":_color })
        World.render(display, display_camera, "display", { "border":_border })
        World.render(ui, ui_camera, "ui")
      }

  } //new

  screen_to_world(mx, my) {

    var region = game_region
    var ratio_x = region.z / width
    var ratio_y = region.w / height

    mx = (mx - region.x) / ratio_x
    my = (my - region.y) / ratio_y

    my = my.floor
    mx = mx.floor

    return Camera.screen_point_to_world(camera, mx, my)

  } //screen_to_world
  
  resize_listen(fn) {
    if(!fn) return
    var uuid = ID.uuid()
    _on_resize[uuid] = fn
    return uuid
  }

  resize_unlisten(id) {
    _on_resize.remove(id)
  }

} //
