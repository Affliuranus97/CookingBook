$(".searchList").click(function(){
    $(this).find(".arrow").toggleClass("rotateImage");
    if($("#priceButton").find(".arrow").hasClass("rotateImage")){
        $("#hiddenPrice").removeClass("hidden");
    }
    else{
        $("#hiddenPrice").addClass("hidden");
    }
    if($("#locationButton").find(".arrow").hasClass("rotateImage")){
        $("#hiddenLocation").removeClass("hidden");
    }
    else{
        $("#hiddenLocation").addClass("hidden");
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
    ShowResults();
};

var sizeSlider = document.getElementById("slideBarSize");
var sizeBullet = document.getElementById("sizeValue");
sizeBullet.innerHTML = sizeSlider.value;

sizeSlider.oninput = function(){
    sizeBullet.innerHTML = this.value;
    var bulletPosition = ((sizeSlider.value - sizeSlider.min) / (sizeSlider.max - sizeSlider.min));
    sizeBullet.style.left = (bulletPosition * 278) + "px";
    ShowResults();
};

var yearSlider = document.getElementById("slideBarYear");
var yearBullet = document.getElementById("yearValue");
yearBullet.innerHTML = yearSlider.value;

yearSlider.oninput = function(){
    yearBullet.innerHTML = this.value;
    var bulletPosition = ((yearSlider.value - yearSlider.min) / (yearSlider.max - yearSlider.min));
    yearBullet.style.left = (bulletPosition * 278) + "px";
    ShowResults();
};

// var floorSlider = document.getElementById("slideBarFloor");
// var floorBullet = document.getElementById("floorValue");
// floorBullet.innerHTML = floorSlider.value;
//
// floorSlider.oninput = function(){
//     floorBullet.innerHTML = this.value;
//     var bulletPosition = ((floorSlider.value - floorSlider.min) / (floorSlider.max - floorSlider.min));
//     floorBullet.style.left = (bulletPosition * 257) + "px";
//     ShowResults();
// };


function CityFilter() {
    if(!$("#filter1").prop("checked")) {
        return undefined;
    }
    let city = $("#loc_city").val();

    if (city.length > 0)
        return city;
    else
        return undefined;
}

function DistrictFilter() {
    if(!$("#filter1").prop("checked")) {
        return undefined;
    }

    let distr = $("#loc_district").val();

    if (distr.length > 0)
        return distr;
    else
        return undefined;
}

function KindFilter() {
    if(!$("#filter2").prop("checked")) {
        return undefined;
    }
    return $("#test1").prop("checked") ? "sale" : "rent";
}

function TypeFilter() {
    if(!$("#filter3").prop("checked")) {
        return undefined;
    }
    return $("#test3").prop("checked") ? "house" : (
        $("#test4").prop("checked") ? "apartment" : "house floor"
    );
}

function ConstructionFilter() {
    if(!$("#filter4").prop("checked")) {
        return undefined;
    }
    return $("#test6").prop("checked") ? "bricks" : "panels";
}

function YearFilter() {
    if(!$("#filter5").prop("checked")){
        return undefined;
    }
    return $("#slideBarYear").val();
}

function PriceFilter() {
    if(!$("#filter8").prop("checked")){
        return undefined;
    }
    return $("#slideBarPrice").val();
}

function SizeFilter() {
    if(!$("#filter9").prop("checked")){
        return undefined;
    }
    return $("#slideBarSize").val();
}


function RunFilters() {
    let city = CityFilter();
    let district = DistrictFilter();
    let maxPrice = PriceFilter();
    let minRooms = undefined;
    let minSize = SizeFilter();
    let minYear = YearFilter();
    let type = TypeFilter();
    let construction = ConstructionFilter();
    let kind = KindFilter();

    let results = [];

    for (let re of HS.RealEstate) {
        if (   (maxPrice     === undefined || re.price         <=  maxPrice    )
            && (city         === undefined || re.city          === city        )
            && (district     === undefined || re.district.includes(district)   )
            && (minYear      === undefined || re.year          >=  minYear     )
            && (minRooms     === undefined || re.rooms         >=  minRooms    )
            && (minSize      === undefined || re.size          >=  minSize     )
            && (type         === undefined || re.type          === type        )
            && (kind         === undefined || re.kind          === kind        )
            && (construction === undefined || re.construction === construction)
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

    if (val < min || val > max){
        element.attr("value", min);
        valText.html(min);
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

    for (let id in filtered) {
        let re = filtered[id];
        results += '<div class="re-property">';
        results += `<a href="view_property.html?id=${id}" target="_blank"><img class="re-image" alt="prop-image" src="${re.images[0]}"></a>`;
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

for(let i = 1; i <= 9; i++)
    $(`#filter${i}`).on('click', ShowResults);

for(let i = 1; i <= 7; i++)
    $(`#test${i}`).on('click', ShowResults);


$("#loc_city").on('change', ShowResults);
$("#loc_district").on('change', ShowResults);
ShowResults();





