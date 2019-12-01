$(".searchList").click(function(){
    $(this).find(".arrow").toggleClass("rotateImage");
    if($("#priceButton").find(".arrow").hasClass("rotateImage")){
        $("#hiddenPrice").removeClass("hidden");
    }
    else{
        $("#hiddenPrice").addClass("hidden");
    }
    if($("#sizeButton").find(".arrow").hasClass("rotateImage")){
        $("#hiddenSize").removeClass("hidden");
    }
    else{
        $("#hiddenSize").addClass("hidden");
    }
    if($("#kindButton").find(".arrow").hasClass("rotateImage")){
        $("#hiddenKind").removeClass("hidden");
    }
    else{
        $("#hiddenKind").addClass("hidden");
    }
    if($("#typeButton").find(".arrow").hasClass("rotateImage")){
        $("#hiddenType").removeClass("hidden");
    }
    else{
        $("#hiddenType").addClass("hidden");
    }
    if($("#constButton").find(".arrow").hasClass("rotateImage")){
        $("#hiddenConst").removeClass("hidden");
    }
    else{
        $("#hiddenConst").addClass("hidden");
    }
    if($("#yearButton").find(".arrow").hasClass("rotateImage")){
        $("#hiddenYear").removeClass("hidden");
    }
    else{
        $("#hiddenYear").addClass("hidden");
    }
    if($("#roomButton").find(".arrow").hasClass("rotateImage")){
        $("#hiddenRoom").removeClass("hidden");
    }
    else{
        $("#hiddenRoom").addClass("hidden");
    }
    if($("#floorButton").find(".arrow").hasClass("rotateImage")){
        $("#hiddenFloor").removeClass("hidden");
    }
    else{
        $("#hiddenFloor").addClass("hidden");
    }
});


var priceSlider = document.getElementById("slideBarPrice");
var priceBullet = document.getElementById("priceValue");
priceBullet.innerHTML = priceSlider.value;

priceSlider.oninput = function(){
    priceBullet.innerHTML = this.value;
    var bulletPosition = ((priceSlider.value - priceSlider.min) / (priceSlider.max - priceSlider.min));
    priceBullet.style.left = (bulletPosition * 278) + "px";
    RunFilters();
};

var sizeSlider = document.getElementById("slideBarSize");
var sizeBullet = document.getElementById("sizeValue");
sizeBullet.innerHTML = sizeSlider.value;

sizeSlider.oninput = function(){
    sizeBullet.innerHTML = this.value;
    var bulletPosition = ((sizeSlider.value - sizeSlider.min) / (sizeSlider.max - sizeSlider.min));
    sizeBullet.style.left = (bulletPosition * 278) + "px";
    RunFilters();
};

var yearSlider = document.getElementById("slideBarYear");
var yearBullet = document.getElementById("yearValue");
yearBullet.innerHTML = yearSlider.value;

yearSlider.oninput = function(){
    yearBullet.innerHTML = this.value;
    var bulletPosition = ((yearSlider.value - yearSlider.min) / (yearSlider.max - yearSlider.min));
    yearBullet.style.left = (bulletPosition * 278) + "px";
    RunFilters();
};

var floorSlider = document.getElementById("slideBarFloor");
var floorBullet = document.getElementById("floorValue");
floorBullet.innerHTML = floorSlider.value;

floorSlider.oninput = function(){
    floorBullet.innerHTML = this.value;
    var bulletPosition = ((floorSlider.value - floorSlider.min) / (floorSlider.max - floorSlider.min));
    floorBullet.style.left = (bulletPosition * 278) + "px";
    RunFilters();
};


HS = {
    Kinds: ["sale", "rent"],
    Types: ["apartment", "house", "house floor"],
    Constructions: ["panels", "bricks"],
    RealEstate: [
        {
            kind: "sale",
            city: "Varna",
            district: "Levski",
            type: "apartment",
            price: 100000,
            floor: 3,
            floors: 6,
            rooms: 3,
            title: "",
            size: 40,
            year: 1999,
            images: [
                "https://q-cf.bstatic.com/images/hotel/max1024x768/134/134203664.jpg",
            ],
            construction: "panels",
        },
        {
            kind: "rent",
            city: "Varna",
            district: "Vinitsa",
            type: "apartment",
            price: 400,
            floor: 1,
            year: 2004,
            floors: 4,
            rooms: 2,
            title: "",
            size: 36,
            images: [
                "https://q-cf.bstatic.com/images/hotel/max1024x768/134/134203664.jpg",
            ],
            construction: "bricks",
        }
    ]
};


function RunFilters() {
    let maxPrice = undefined;
    let city = undefined;
    let district = undefined;

    let results = [];

    for (let re of HS.RealEstate) {
        if (
               (maxPrice === undefined || re.price <= maxPrice)
            && (    city === undefined || re.city === city    )
            && (district === undefined || re.city === district)
        ) {
            results.push(re);
        }
    }

    return results;
}

function NormalizeSlider(element, minText, maxText, valText, property) {
    let min = HS.RealEstate[0][property];
    let max = min;

    for (let re of HS.RealEstate) {
        if (re[property] < min){
            min = re[property];
        }
        if (re[property] > max){
            max = re[property];
        }
    }

    let val = parseInt(element.attr("value"));

    element.attr("min", min);
    element.attr("max", max);

    minText.html(min);
    maxText.html(max);

    if (val < min){
        element.attr("value", min);
        valText.html(min);
    } else if (val > max){
        element.attr("value", max);
        valText.html(max);
    }
}

function NormalizeSliderAuto(name){
    let titleName = name.charAt(0).toUpperCase() + name.substring(1);

    let label = $(`.${name}Label`);
    NormalizeSlider(
        $(`#slideBar${titleName}`),
        label.find("span:first-child"),
        label.find("span:last-child"),
        $(`#${name}Value`),
        name
    );
}

function ShowResults() {
    NormalizeSliderAuto('year');
    NormalizeSliderAuto('floor');
    NormalizeSliderAuto('size');
    NormalizeSliderAuto('price');

    let filtered = RunFilters();
    let results = "";

    let target = $(".searchResults");

    for (let re of filtered) {
        results += '<div class="re-property">';
        results += `<img class="re-image" alt="prop-image" src="${re.images[0]}">`;
        results += `<div><span class="re-title">${re.title}</span></div>`;
        results += `<div><span class="re-label">Price:</span><span class="re-value">${re.price} лв</span></div>`;
        results += `<div><span class="re-label">District:</span><span class="re-value">${re.district}</span></div>`;
        results += `<div><span class="re-label">Type:</span><span class="re-value">${re.type}</span></div>`;
        results += `<div><span class="re-label">Rooms:</span><span class="re-value">${re.rooms}</span></div>`;
        results += `<div><span class="re-label">Floor:</span><span class="re-value">${re.floor}/${re.floors}</span></div>`;
        results += '</div>';
    }

    target.html(results);
}

ShowResults();





