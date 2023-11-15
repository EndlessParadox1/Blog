let path = location.pathname;
let top_ = $('#top_');
let header = $('#header');
let content = $('#content');
let archive = $('#archive');

index();

function index() {
    fetch('/api' + path)
        .then(res => res.json())
        .then(data => {
            if(data.hasOwnProperty('msg'))
                alert('Load failed: ' + data.msg);
            else {
                top_.html(path.replace('/user/', '') + "'s Blog");
                header.html('Latest 10');
                content.html('');
                archive.html('');
                if(data.hasOwnProperty('ids')) {
                    let ids = data.ids;
                    let titles = data.titles;
                    let summaries = data.summaries;
                    let times = data.times;
                    for (let i = 0; i < ids.length; i++) {
                        let tmp = `<article class="blog-post">` +
                            `<h2 class="blog-post-title">${titles[i]}</h2>` +
                            `<p class="blog-post-meta">Post on ${times[i]}</p>` +
                            `${summaries[i]}<hr><div id="D${ids[i]}" class="detail" data-flag="0"></div>` +
                            `<div id="B${ids[i]}" class="btn btn-sm btn-light" onclick="detail(${ids[i]})">Unfold</div></article>`;
                        content.append(tmp);
                    }
                    for (let arch of data.archs) {
                        let tmp = `<li><a onclick="list(${arch})" class="link">${arch}</a></li>`;
                        archive.append(tmp);
                    }
                } else
                    content.html('There is nothing yet.');
            }
        });
}

function detail(i) {
    let div = $('#D' + i);
    let btn = $('#B' + i);
    div.slideToggle('fast');
    if(btn.html() === 'Unfold')
        btn.html('Fold')
    else
        btn.html('Unfold');
    if(div.attr('data-flag') === '0') {
        fetch(`/api/topic/${i}?level=0`)
            .then(res => res.json())
            .then(data => {
                if(data.hasOwnProperty('msg'))
                    alert('Load failed: '+ data.msg);
                else {
                    div.html(data.html);
                    div.attr('data-flag', '1');
                }
            })
    }
}

function list(dt) {
    fetch(`/api${path}/archive/${dt.replace('~', '-')}`)
        .then(res => res.json())
        .then(data => {
            if (data.hasOwnProperty('msg'))
                alert('Load failed: ' + data.msg);
            else {
                header.html(dt);
                content.html('');
                if (data.hasOwnProperty('ids')) {
                    let ids = data.ids;
                    let titles = data.titles;
                    let summaries = data.summaries;
                    let times = data.times;
                    for (let i = 0; i < ids.length; i++) {
                        let tmp = `<article class="blog-post">` +
                            `<h2 class="blog-post-title">${titles[i]}</h2>` +
                            `<p class="blog-post-meta">Post on ${times[i]}</p>` +
                            `${summaries[i]}<hr><div id="D${ids[i]}" class="detail"></div>` +
                            `<div id="B${ids[i]}" class="btn btn-sm btn-light" onclick="detail(${ids[i]})">Unfold</div></article>`;
                        content.append(tmp);
                    }
                } else
                    content.html('Lost in time travel.');
            }
        });
}
