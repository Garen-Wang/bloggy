window.onload = function() {
    let container = document.querySelector('#toc-aside');
    if (container != null) {
        resize_toc(container);
        toc_scroll_position(container);
        window.onscroll = function() { toc_scroll_position(container) };
    }
}

function resize_toc(container) {
    let containerHeight = container.clientHeight;

    let resize = function() {
        if (containerHeight > document.documentElement.clientHeight - 100) {
            container.classList.add('coarse');
        } else {
            container.classList.remove('coarse');
        }
    };
    resize();

    let resizeId;
    window.onresize = function() {
        clearTimeout(resizeId);
        resizeId = setTimeout(resize, 300);
    };
}

function toc_scroll_position(container) {
    if (container.offsetParent === null) {
        // skip computation if ToC is not visible
        return;
    }

    // remove active class for all items
    for (item of container.querySelectorAll("li")) {
        item.classList.remove("active");
    }

    // look for active item
    let site_offset = document.documentElement.scrollTop;
    let current_toc_item = null;
    for (item of container.querySelectorAll("li")) {
        if (item.offsetParent === null) {
            // skip items that are not visible
            continue;
        }
        let anchor = item.firstElementChild.getAttribute("href");
        let heading = document.querySelector(anchor);
        if (heading.offsetTop <= (site_offset + document.documentElement.clientHeight / 3)) {
            current_toc_item = item;
        } else {
            break;
        }
    }

    // set active class for current ToC item
    if (current_toc_item != null) {
        current_toc_item.classList.add("active");
    }
}