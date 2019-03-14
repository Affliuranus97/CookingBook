var data = [
	{
		"id": 0,
		"name": "kiufet",
		"description": "blq blq desc",
		"ingredients": [{"name": "sol", "amount": "100"}],
		"instruction": "instrukcii"
	},
	{
		"id": 1,
		"name": "supa",
		"description": "supa desc",
		"ingredients": [{"name": "sol", "amount": "100"},{"name": "voda", "amount": "200"}],
		"instruction": "supa instrukcii"
	}
	];
	
function createCard(object){
	return '<div class="card" id="recipe_' + object["id"] + '"><div class="image"></div>' +
	'<div class="title">' + object["name"] + '</div></div>';
};

function fillCards(objects){
	let cardString = "";
	for(let i = 0; i < objects.length; i++){
		cardString += createCard(objects[i]);
	}
	$(".results").html(cardString);
}

fillCards(data);

//TODO: id.split("_"), vzemam vtoriq string ot spisuka, pravq go na int i tursq
//v obekta porednostta

$(".card").click(function(){
	/*$(this).toggleClass("clicked");*/
	$(".wrapper").addClass("visible");
});

$(".wrapper").click(function(e){
	if (e.target === this) {
		$(this).removeClass("visible");
	}
});