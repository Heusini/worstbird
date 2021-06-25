function expand(elem) {
    // let x = document.getElementById("asdf");
    // x.style.webkitLineClamp = none;
    if (elem.style.webkitLineClamp === "none") {
        elem.style.setProperty("-webkit-line-clamp", "9");
    } else {
        elem.style.setProperty("-webkit-line-clamp", "none");
    }
}