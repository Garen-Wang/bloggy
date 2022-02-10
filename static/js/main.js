window.onload = function() {
    let container = document.querySelector('#toc-aside');
    if (container != null) {
        generate_anchors(container);
        resize_toc(container);
        toc_scroll_position(container);
        window.onscroll = function() { toc_scroll_position(container) };
    }
}

// function getElementByXpath(xpath) {
//     return document.evaluate(xpath, document, null, XPathResult.FIRST_ORDERED_NODE_TYPE, null).singleNodeValue;
// }

function generate_anchors(container) {
    let article_titles = document.querySelector('main').querySelectorAll('h1, h2, h3, h4, h5, h6');
    let toc_titles = container.querySelectorAll('li');
    let new_article_titles = Array.from(article_titles).slice(1, -2);
    if (new_article_titles.length === toc_titles.length) {
        toc_titles.forEach((toc_title, idx) => {
            let article_title = new_article_titles[idx];
            article_title.setAttribute("id", toc_title.firstElementChild.getAttribute("href").substring(1));
        });
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
        // let decoded_uri = decodeURI(anchor);
        // console.log(decoded_uri);
        let heading = document.querySelector('[id="' + anchor.substring(1) + '"]');
        // console.log(heading);
        if (heading.offsetTop <= site_offset) {
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