let path = location.pathname;
let top_ = $('#top_');
let header = $('#header');
let content = $('#content');
let archive = $('#archive');

$(async () => await index())

async function index() {
    let res = await fetch('/api' + path);
    let data = await res.json();
    if(!res.ok)
        alert(data.msg);
    else {
        top_.html(path.replace('/user/', '') + "'s Blog");
        header.html('Latest 10');
        content.html('');
        archive.html('');
        if(!$.isEmptyObject(data)) {
            let ids = data.ids;
            let titles = data.titles;
            let summaries = data.summaries;
            let times = data.times;
            for (let i = 0; i < ids.length; i++) {
                let tmp = `<article class="blog-post">` +
                    `<h2 class="blog-post-title">${titles[i]}</h2>` +
                    `<p class="blog-post-meta">Post on ${times[i]}</p>` +
                    `${summaries[i]}<hr><div id="C${ids[i]}" class="detail" data-flag="0"></div>` +
                    `<div id="B${ids[i]}" class="btn btn-sm btn-light">Unfold</div></article>`;
                content.append(tmp);
                $('#B' + ids[i]).click(async () => await detail(ids[i]));
            }
            for (let arch of data.archs) {
                let tmp = `<li><a id="${arch}" class="link">${arch}</a></li>`;
                archive.append(tmp);
                $(`#${arch}`).click(async () => await list(arch));
            }
        } else
            content.html('There is nothing yet.');
    }
}

async function list(dt) {
    let res = await fetch(`/api${path}/archive/${dt}`);
    let data = await res.json();
    if (!res.ok)
        alert('Load failed: ' + data.msg);
    else {
        header.html(dt);
        content.html('');
        if(!$.isEmptyObject(data)) {
            let ids = data.ids;
            let titles = data.titles;
            let summaries = data.summaries;
            let times = data.times;
            for (let i = 0; i < ids.length; i++) {
                let tmp = `<article class="blog-post">` +
                    `<h2 class="blog-post-title">${titles[i]}</h2>` +
                    `<p class="blog-post-meta">Post on ${times[i]}</p>` +
                    `${summaries[i]}<hr><div id="C${ids[i]}" class="detail" data-flag="0"></div>` +
                    `<div id="B${ids[i]}" class="btn btn-sm btn-light">Unfold</div></article>`;
                content.append(tmp);
                $('#B' + ids[i]).click(async () => await detail(ids[i]));
            }
        } else
            content.html('Lost in time travel.');
    }
}

async function detail(i) {
    let div = $('#C' + i);
    let btn = $('#B' + i);
    div.slideToggle('fast');
    if(btn.html() === 'Unfold')
        btn.html('Fold')
    else
        btn.html('Unfold');
    if(div.attr('data-flag') === '0') {
        let res = await fetch(`/api/topic/${i}?level=0`);
        let data = await res.json();
        if(!res.ok)
            alert('Load failed: ' + data.msg);
        else {
            div.html(data.html);
            div.attr('data-flag', '1');
        }
    }
}
