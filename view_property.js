let params = window.location.search.substr(1);
let id = parseInt(params.substr(3));
let re = HS.RealEstate[id];

function upper(l){
    return l.toString().charAt(0).toUpperCase() + l.toString().substr(1);
}

for(let k in re){
    $(`#re-${k}`).html(k === 'image' ? re[k] : upper(re[k]));
}

$(".main-image").attr("src", re.images[0]);
