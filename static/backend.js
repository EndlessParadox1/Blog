let path = location.pathname;
let header = $('#header');
let content = $('#content');
let table = '<div class="mb-2">' +
    '<label for="title" class="form-label">Title</label>' +
    '<input type="text" id="title" class="form-control" placeholder="60 general characters at most" maxlength="60">' +
    '<label for="summary" class="form-label">Summary</label>' +
    '<textarea id="summary" class="form-control" placeholder="300 general characters at most" maxlength="300" rows="3"></textarea>' +
    '<label for="markdown" class="form-label">Content</label>' +
    '<textarea id="markdown" class="form-control" placeholder="Please write in Markdwon format" rows="15"></textarea></div>' +
    '<div style="text-align: center">';

fetch('/api' + path)
    .then(res => {
        if(res.ok) {
            $('#home').attr('href', path.replace('admin', 'user'));
            header.html('Hi, ' + path.replace('/admin/', ''));
            $('#logout').click(() => logout());
            $('#new').click(() => new_());
            $('#all').click(() => list());
        } else
            return res.json();
    })
    .then(err => alert(err.msg));

function logout() {
    let ans = confirm('Sure to sign out?');
    if(ans) {
        fetch('/api/logout')
            .then(res => {
                if(res.status !== 300)
                    return res.json();
            })
            .then(err => alert('Logout failed: ' + err.msg));
    }
}

function new_() {
    header.html('New Blog');
    content.html(table + '<button id="postBtn" class="btn btn-primary">Post</button></div>');
    $('#postBtn').click(() => {
        let res = confirm('Sure to post?');
        if(res) {
            let title = $('#title').val();
            let summary = $('#summary').val();
            let markdown = $('#markdown').val();
            if (!/^[\x20-\x7E]{1,60}$/.test(title) || !/^[\x20-\x7E]{1,300}$/.test(summary) || markdown.empty()) {
                alert('Some field is wrong!');
                return;
            }
            fetch(`/api${path}/topic/`, {
                method: 'POST',
                headers: {'Content-Type': 'application/json'},
                body: JSON.stringify({
                    'title': title,
                    'summary': summary,
                    'markdown': markdown
                })
            })
                .then(res => {
                    if (res.ok)
                        new_(); // fresh
                    else
                        return res.json();
                })
                .then(err => alert('Post failed: ' + err.msg));
        }
        });
}

function list() {
    header.html('All Blogs');
    content.html('');
    fetch(`/api${path}/topic`)
        .then(res => res.json())
        .then(data => {
            if (data.hasOwnProperty('msg'))
                alert('List failed: ' + data.msg);
            else {
                if(data.hasOwnProperty('ids')) {
                    let tmp = '<table class="table table-striped table-hover">' +
                        '<thead style="text-align: center"><tr><th>No.</th><th>Title</th><th>Opration</th><th>Detail</th></tr></thead>' +
                        '<tbody id="body"></tbody></table>';
                    content.html(tmp);
                    let i = 1;
                    let ids = data.ids;
                    let titles = data.titles;
                    let body = $('#body');
                    for (let k = 0; k < ids.length; k++) {
                        tmp = `<tr class="trow"><td>#${i}</td><td>${titles[k]}</td>` +
                            `<td><div class="btn btn-info btn-sm" onclick="edit(${ids[k]})">Modify</div>` +
                            `<div class="btn btn-danger btn-sm" onClick="del(${ids[k]})">Delete</div></td>` +
                            `<td><div id="B${ids[k]}" class="btn btn-light btn-sm" onclick="detail(${ids[k]})">▲</div></td></tr>` +
                            `<tr><td rowspan="4"><div id="D${ids[k]}" data-flag="0" class="detail"></div></div></td></tr>`;
                        body.append(tmp);
                        i++;
                    }
                } else
                    content.html('There is nothing yet.');
            }
        })
}

function edit(i) {
    header.html('Modify Blog');
    fetch(`/api${path}/topic/${i}?level=1`)
        .then(res => res.json())
        .then(data => {
            if(data.hasOwnProperty('msg'))
                alert('Load failed: ' + data.msg);
            else {
                let title = $('#title');
                let summary = $('#summary');
                let markdown = $('#markdown');
                content.html(table + '<button id="submitBtn" class="btn btn-primary">Submit</button></div>');
                title.val(data.title);
                summary.val(data.summary);
                markdown.val(data.markdown);
                $('#submitBtn').click(() => {
                    let res = confirm('Sure to submit?');
                    if(res) {
                        let title = $('#title').val();
                        let summary = $('#summary').val();
                        let markdown = $('#markdown').val();
                        if (!/^[\x20-\x7E]{1,60}$/.test(title) || !/^[\x20-\x7E]{1,300}$/.test(summary) || markdown.empty()) {
                            alert('Some field is wrong!');
                            return;
                        }
                        fetch(`/api${path}/topic/${i}`, {
                            method: 'POST',
                            headers: {'Content-Type': 'application/json'},
                            body: JSON.stringify({
                                'title': title,
                                'summary': summary,
                                'markdown': markdown
                            })
                        })
                            .then(res => {
                                if (res.ok)
                                    list(); // fresh
                                else
                                    return res.json();
                            })
                            .then(err => alert('Update failed: ' + err.msg));
                    }
                });
            }
        })

}

function del(i) {
    let ans = confirm('Sure to delete「{{ item.name }}」?');
    if(ans) {
        fetch(`/api${path}/topic/` + i, {method: 'DELETE'})
            .then(res => {
                if(res.ok)
                    list(); // fresh
                else
                    return res.json();
            })
            .then(err => alert('Delete failed: ' + err.msg));
    }
}

function detail(i) {
    let div = $('#D' + i);
    let btn = $('#B' + i);
    if(div.attr('data-flag') === '0') {
        fetch(`/api/topic/${i}?level=2`)
            .then(res => res.json())
            .then(data => {
                if(data.hasOwnProperty('msg'))
                    alert('Load failed: ' + data.msg);
                else {
                    div.html(data.html);
                    div.attr('data-flag', '1');
                }
            })
    }
    if(div.attr('data-flag') === '0')
        return;
    div.slideToggle('fast');
    if(btn.html() === '▲')
        btn.html('▼')
    else
        btn.html('▲');
}
