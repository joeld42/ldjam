import os, sys, string, random
import requests
import itertools

CARDS_TSV_URL = "https://docs.google.com/spreadsheets/d/e/2PACX-1vR7Pem4YtyzLJCaBOQJvKdnCkLiPNhf7i9p1twv8le0Xx_qmbmXdnuOyVY_gfyq_1U5o-gkpZDj-u39/pub?gid=0&single=true&output=tsv"
CARD_TSV_CACHED = "./srcart/card_data.tsv"
CARDS_DATA_DIR = "./signal/cards/"
#CARDS_DATA_DIR = "./tmp"

def generate_id(length=6):
	characters = string.ascii_letters + string.digits
	return ''.join(random.choices(characters, k=length))

def extract_outcomes( data ):
	
	outcomes = data.split( "Cmd:" )
	# print ("Extract Outcomes: ", outcomes )
	prompt = outcomes[0]
	cmds = []
	for cmd in outcomes[1:]:
		cmds.append( cmd.replace( "Cmd:", "" ).strip() )


	return (prompt, ";".join(cmds) )



def dump_cards( cards  ):

	for card in cards:

		id = card['Card Id']

		# Get outcomes out of data
		oc_right = extract_outcomes( card['PromptRight'] )
		oc_left = extract_outcomes( card['PromptLeft'] )

		outcomes = []
		if not oc_right[0]:
			print(f"WARNING: '{id}' missing default response prompt")
		outcomes.append( oc_right)

		# Left prompt is optional
		if oc_left[0]:
			outcomes.append( oc_left )
		

		# Write card file
		cardfile = os.path.join ( CARDS_DATA_DIR, f"{id}.card.lx")		
		with open( cardfile, "w") as fp:
			fp.write("card = {\n")
			
			title = card['Title']
			fp.write(f'  title = "{title}"\n')

			text = card['Text']
			fp.write(f'  text = "{text}"\n')

			if len(outcomes):
				fp.write('    outcomes = [')
				for oc in outcomes:
					ocid = generate_id()
					fp.write('      {\n')
					fp.write(f'         id = "{ocid}"\n')
					fp.write('         value = {\n')
					fp.write(f'            prompt = "{oc[0]}"\n')
					fp.write(f'            cmds = "{oc[1]}"\n')
					fp.write('         } // value\n')
					fp.write('      } // outcome\n')
				fp.write('    ] // outcomes')

			fp.write("} // card\n\n")
	

def parse_cards( lines ):
	cards = []

	labels = lines[0].split("\t")
	for line in lines[1:]:
		items = line.split("\t")

		# print("labels", labels )
		# print("items", items )

		cardData = {}
		for (k,v) in itertools.zip_longest( labels, items ):
			# print ("k:", k, v )
			cardData[k.strip()] = v.strip()

		cards.append( cardData )

	return cards



if __name__=='__main__':
	lines = []
	cards = []

	if "--use-cached" in sys.argv:
		with open( CARD_TSV_CACHED, "rt") as fp:
			lines = fp.readlines()
		print("Using cached data ", CARD_TSV_CACHED )
	else:
		response = requests.get( CARDS_TSV_URL )
		if response.status_code == 200:

			with open( CARD_TSV_CACHED, "wt") as fp:
				fp.write(response.text)
			print("Cached ", CARD_TSV_CACHED )

			lines = response.text.splitlines()
			
		else:
			print(f"Error: {response.status_code}")

	cards = parse_cards( lines )
	dump_cards( cards )




