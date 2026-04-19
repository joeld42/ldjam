import "luxe: assets" for Assets
import "luxe: asset/type" for AssetType, AssetContext
import "luxe: io" for IO

#block = type
class CardInfo {
    var title: String = "Card Title"
    var text: String = "Card text goes here"

    #count = 1
    #type = CardOutcome
    var outcomes: List = []

}

class CardOutcome {
  var prompt: String = "Okay"
  var cmds: String = ""
}

class Asset is AssetType {
  subtype { "card" }
  construct new(type_id: String, ctx: AssetContext) {
    super(type_id, ctx)
  }
} //Asset