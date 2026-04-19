class Card {

    title { _title }
    text { _text }

    construct new ( title, card_data ) {
        _title = title
        _text = card_data["text"]

        _options = []
        if (card_data.containsKey("options")) {
            for (option_data in card_data["options"]) {

                // Just stuff the options in directly, since this is a game jam
                // May want to unpack them laters
                _options.add( option_data )
            }
        }
    }

}