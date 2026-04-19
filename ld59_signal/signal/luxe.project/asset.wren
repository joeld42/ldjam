//This file is generated. Do not edit directly

import "luxe: assets" for Assets, Strings
import "luxe: blocks" for Block

import "luxe: type/anim.asset" for AnimDesc
import "luxe: type/asset.meta.asset" for AssetMeta
import "luxe: type/audio.asset" for AudioDesc
import "luxe: type/basis.asset" for Basis
import "luxe: type/audio.bus.asset" for AudioBusMeta
import "type/card.asset" for CardInfo
import "luxe: type/clip.asset" for ClipDesc
import "luxe: type/compute.asset" for Compute
import "luxe: type/font.face.asset" for FaceDesc
import "luxe: type/font.asset" for FontDesc
import "luxe: type/mesh.asset" for MeshDesc
import "luxe: type/scene.asset" for SceneDesc
import "luxe: type/skeleton.asset" for SkeletonDesc
import "luxe: type/stage.asset" for StageDesc
import "luxe: type/ui/style.asset" for Style
import "luxe: type/text.style.asset" for TextStyle
import "luxe: type/ui/theme.asset" for Theme
import "luxe: type/tiles.asset" for TilesDesc
import "luxe: type/tiles.brush.asset" for TilesBrush
import "luxe: type/tiles.visual.asset" for TilesVisualDesc

class AssetType {
  static anim { "luxe: type/anim.asset" }
  static anim_track { "luxe: type/anim.track.asset" }
  static asset_meta { "luxe: type/asset.meta.asset" }
  static atlas { "luxe: type/atlas.asset" }
  static audio { "luxe: type/audio.asset" }
  static basis { "luxe: type/basis.asset" }
  static block { "luxe: type/block.asset" }
  static block_def { "luxe: type/block_def.asset" }
  static bus { "luxe: type/audio.bus.asset" }
  static card { "type/card.asset" }
  static clip { "luxe: type/clip.asset" }
  static compute { "luxe: type/compute.asset" }
  static control { "luxe: type/ui/control.asset" }
  static entity { "luxe: type/entity.asset" }
  static face { "luxe: type/font.face.asset" }
  static field_display { "luxe: type/editor/field_display.asset" }
  static font { "luxe: type/font.asset" }
  static image { "luxe: type/image.asset" }
  static material { "luxe: type/material.asset" }
  static mesh { "luxe: type/mesh.asset" }
  static mesh_preset { "luxe: type/mesh.preset.asset" }
  static modifier { "luxe: type/modifier.asset" }
  static pose_node { "luxe: type/pose_node.asset" }
  static prototype { "luxe: type/prototype.asset" }
  static scene { "luxe: type/scene.asset" }
  static skeleton { "luxe: type/skeleton.asset" }
  static stage { "luxe: type/stage.asset" }
  static style { "luxe: type/ui/style.asset" }
  static style_type { "luxe: type/ui/style_type.asset" }
  static task { "luxe: type/task.asset" }
  static text_style { "luxe: type/text.style.asset" }
  static theme { "luxe: type/ui/theme.asset" }
  static tiles { "luxe: type/tiles.asset" }
  static tiles_brush { "luxe: type/tiles.brush.asset" }
  static tiles_visual { "luxe: type/tiles.visual.asset" }
  static vfx_module { "luxe: type/vfx/module.asset" }
  static vfx_render { "luxe: type/vfx/render.asset" }
}

class AssetGetAPI {
  construct new() {}
  #asset_type(arg = "asset_id", subtype= "anim")
  anim(asset_id: String) : AnimDesc { Asset.instance(AssetType.anim, asset_id) }
  #asset_type(arg = "asset_id", subtype= "asset_meta")
  asset_meta(asset_id: String) : AssetMeta { Asset.instance(AssetType.asset_meta, asset_id) }
  #asset_type(arg = "asset_id", subtype= "audio")
  audio(asset_id: String) : AudioDesc { Asset.instance(AssetType.audio, asset_id) }
  #asset_type(arg = "asset_id", subtype= "basis")
  basis(asset_id: String) : Basis { Asset.instance(AssetType.basis, asset_id) }
  #asset_type(arg = "asset_id", subtype= "bus")
  bus(asset_id: String) : AudioBusMeta { Asset.instance(AssetType.bus, asset_id) }
  #asset_type(arg = "asset_id", subtype= "card")
  card(asset_id: String) : CardInfo { Asset.instance(AssetType.card, asset_id) }
  #asset_type(arg = "asset_id", subtype= "clip")
  clip(asset_id: String) : ClipDesc { Asset.instance(AssetType.clip, asset_id) }
  #asset_type(arg = "asset_id", subtype= "compute")
  compute(asset_id: String) : Compute { Asset.instance(AssetType.compute, asset_id) }
  #asset_type(arg = "asset_id", subtype= "face")
  face(asset_id: String) : FaceDesc { Asset.instance(AssetType.face, asset_id) }
  #asset_type(arg = "asset_id", subtype= "font")
  font(asset_id: String) : FontDesc { Asset.instance(AssetType.font, asset_id) }
  #asset_type(arg = "asset_id", subtype= "mesh")
  mesh(asset_id: String) : MeshDesc { Asset.instance(AssetType.mesh, asset_id) }
  #asset_type(arg = "asset_id", subtype= "scene")
  scene(asset_id: String) : SceneDesc { Asset.instance(AssetType.scene, asset_id) }
  #asset_type(arg = "asset_id", subtype= "skeleton")
  skeleton(asset_id: String) : SkeletonDesc { Asset.instance(AssetType.skeleton, asset_id) }
  #asset_type(arg = "asset_id", subtype= "stage")
  stage(asset_id: String) : StageDesc { Asset.instance(AssetType.stage, asset_id) }
  #asset_type(arg = "asset_id", subtype= "style")
  style(asset_id: String) : Style { Asset.instance(AssetType.style, asset_id) }
  #asset_type(arg = "asset_id", subtype= "text_style")
  text_style(asset_id: String) : TextStyle { Asset.instance(AssetType.text_style, asset_id) }
  #asset_type(arg = "asset_id", subtype= "theme")
  theme(asset_id: String) : Theme { Asset.instance(AssetType.theme, asset_id) }
  #asset_type(arg = "asset_id", subtype= "tiles")
  tiles(asset_id: String) : TilesDesc { Asset.instance(AssetType.tiles, asset_id) }
  #asset_type(arg = "asset_id", subtype= "tiles_brush")
  tiles_brush(asset_id: String) : TilesBrush { Asset.instance(AssetType.tiles_brush, asset_id) }
  #asset_type(arg = "asset_id", subtype= "tiles_visual")
  tiles_visual(asset_id: String) : TilesVisualDesc { Asset.instance(AssetType.tiles_visual, asset_id) }
}
var AssetGet = AssetGetAPI.new()

class AssetAddAPI {
  construct new() {}
  anim(asset_id: String) : AnimDesc { Asset.add(AssetType.anim, asset_id) }
  asset_meta(asset_id: String) : AssetMeta { Asset.add(AssetType.asset_meta, asset_id) }
  audio(asset_id: String) : AudioDesc { Asset.add(AssetType.audio, asset_id) }
  basis(asset_id: String) : Basis { Asset.add(AssetType.basis, asset_id) }
  bus(asset_id: String) : AudioBusMeta { Asset.add(AssetType.bus, asset_id) }
  card(asset_id: String) : CardInfo { Asset.add(AssetType.card, asset_id) }
  clip(asset_id: String) : ClipDesc { Asset.add(AssetType.clip, asset_id) }
  compute(asset_id: String) : Compute { Asset.add(AssetType.compute, asset_id) }
  face(asset_id: String) : FaceDesc { Asset.add(AssetType.face, asset_id) }
  font(asset_id: String) : FontDesc { Asset.add(AssetType.font, asset_id) }
  mesh(asset_id: String) : MeshDesc { Asset.add(AssetType.mesh, asset_id) }
  scene(asset_id: String) : SceneDesc { Asset.add(AssetType.scene, asset_id) }
  skeleton(asset_id: String) : SkeletonDesc { Asset.add(AssetType.skeleton, asset_id) }
  stage(asset_id: String) : StageDesc { Asset.add(AssetType.stage, asset_id) }
  style(asset_id: String) : Style { Asset.add(AssetType.style, asset_id) }
  text_style(asset_id: String) : TextStyle { Asset.add(AssetType.text_style, asset_id) }
  theme(asset_id: String) : Theme { Asset.add(AssetType.theme, asset_id) }
  tiles(asset_id: String) : TilesDesc { Asset.add(AssetType.tiles, asset_id) }
  tiles_brush(asset_id: String) : TilesBrush { Asset.add(AssetType.tiles_brush, asset_id) }
  tiles_visual(asset_id: String) : TilesVisualDesc { Asset.add(AssetType.tiles_visual, asset_id) }
}
var AssetAdd = AssetAddAPI.new()

class Asset {
  #hidden
  static add(type_id: String, asset_id: String) {
    var block = Assets.get_block(type_id)
    if(!Block.valid(block)) return null
    var inst = Block.add(block, Strings.add(asset_id))
    return Block.instance(block, inst)
  }
  #hidden
  static instance(type_id: String, asset_id: String) {
    var block = Assets.get_block(type_id)
    if(!Block.valid(block)) return null
    var inst = Block.get(block, Strings.add(asset_id))
    return Block.instance(block, inst)
  }
  static get : AssetGetAPI { AssetGet }
  static add : AssetAddAPI { AssetAdd }
  #asset_type(arg = "asset_id", subtype= "anim")
  static anim(asset_id: String) : Num { Assets.get_handle(AssetType.anim, asset_id) }
  #asset_type(arg = "asset_id", subtype= "atlas")
  static atlas(asset_id: String) : Num { Assets.get_handle(AssetType.atlas, asset_id) }
  #asset_type(arg = "asset_id", subtype= "audio")
  static audio(asset_id: String) : Num { Assets.get_handle(AssetType.audio, asset_id) }
  #asset_type(arg = "asset_id", subtype= "bus")
  static bus(asset_id: String) : Num { Assets.get_handle(AssetType.bus, asset_id) }
  #asset_type(arg = "asset_id", subtype= "clip")
  static clip(asset_id: String) : Num { Assets.get_handle(AssetType.clip, asset_id) }
  #asset_type(arg = "asset_id", subtype= "compute")
  static compute(asset_id: String) : Num { Assets.get_handle(AssetType.compute, asset_id) }
  #asset_type(arg = "asset_id", subtype= "face")
  static face(asset_id: String) : Num { Assets.get_handle(AssetType.face, asset_id) }
  #asset_type(arg = "asset_id", subtype= "font")
  static font(asset_id: String) : Num { Assets.get_handle(AssetType.font, asset_id) }
  #asset_type(arg = "asset_id", subtype= "prototype")
  static prototype(asset_id: String) : Num { Assets.get_handle(AssetType.prototype, asset_id) }
  #asset_type(arg = "asset_id", subtype= "scene")
  static scene(asset_id: String) : Num { Assets.get_handle(AssetType.scene, asset_id) }
  #asset_type(arg = "asset_id", subtype= "skeleton")
  static skeleton(asset_id: String) : Num { Assets.get_handle(AssetType.skeleton, asset_id) }
  #asset_type(arg = "asset_id", subtype= "stage")
  static stage(asset_id: String) : Num { Assets.get_handle(AssetType.stage, asset_id) }
  #asset_type(arg = "asset_id", subtype= "style")
  static style(asset_id: String) : Num { Assets.get_handle(AssetType.style, asset_id) }
  #asset_type(arg = "asset_id", subtype= "task")
  static task(asset_id: String) : Num { Assets.get_handle(AssetType.task, asset_id) }
  #asset_type(arg = "asset_id", subtype= "text_style")
  static text_style(asset_id: String) : Num { Assets.get_handle(AssetType.text_style, asset_id) }
  #asset_type(arg = "asset_id", subtype= "theme")
  static theme(asset_id: String) : Num { Assets.get_handle(AssetType.theme, asset_id) }
  #asset_type(arg = "asset_id", subtype= "tiles")
  static tiles(asset_id: String) : Num { Assets.get_handle(AssetType.tiles, asset_id) }
  #asset_type(arg = "asset_id", subtype= "tiles_brush")
  static tiles_brush(asset_id: String) : Num { Assets.get_handle(AssetType.tiles_brush, asset_id) }
  #asset_type(arg = "asset_id", subtype= "tiles_visual")
  static tiles_visual(asset_id: String) : Num { Assets.get_handle(AssetType.tiles_visual, asset_id) }
}
