(window.webpackJsonp=window.webpackJsonp||[]).push([[0],[,,function(e,t,n){},function(e,t,n){"use strict";n.r(t);var o=n(0),r=(n(2),n(1));const s=Object(o.h)('<div class="systen_msg"><hr><h3></h3><button class="std_btn">Ok</button><hr>'),i=Object(o.h)('<div class="covering-panel">'),[c,l]=Object(r.j)(),[a,d]=Object(r.j)();function u(){l();const e=a();e&&e(),d()}function b(){return[(()=>{const e=s(),t=e.firstChild.nextSibling,n=t.nextSibling;return Object(o.d)(t,c),n.$$click=u,e})(),i()]}Object(o.c)(["click"]);const h=Object(o.h)('<p style="color:red;font-size:large;"> '),v=Object(o.h)("<p> | <!> | "),f=Object(o.h)("<hr>"),g="auth_token";function j(e,t,n){let o=new URL(e,window.location.href);o.protocol=o.protocol.replace("http","ws");let r=new WebSocket(o);return r.onopen=()=>console.log("socket open on ".concat(o)),r.onmessage=e=>{t(e.data)},r.onclose=e=>{n&&n(e)},r}function p(e){let t=arguments.length>1&&void 0!==arguments[1]?arguments[1]:g,n=arguments.length>2&&void 0!==arguments[2]?arguments[2]:7;const o=new Date;o.setTime(o.getTime()+24*n*60*60*1e3),document.cookie=t+"="+e+";expires="+o.toUTCString()+";path=/"}function O(e){return[(()=>{const t=h(),n=t.firstChild;return Object(o.d)(t,()=>{var t;return null===(t=e.stats)||void 0===t?void 0:t.username},n),t})(),(()=>{const t=v(),n=t.firstChild,r=n.nextSibling;r.nextSibling;return Object(o.d)(t,()=>{var t;return"Wins: "+(null===(t=e.stats)||void 0===t?void 0:t.wins)},n),Object(o.d)(t,()=>{var t;return"Loses: "+(null===(t=e.stats)||void 0===t?void 0:t.loses)},r),Object(o.d)(t,()=>{var t,n,o,r;return"K/D: "+(o=null===(t=e.stats)||void 0===t?void 0:t.wins,r=null===(n=e.stats)||void 0===n?void 0:n.loses,o&&r?o/(o+r):0)},null),t})(),f()]}const m=Object(o.h)('<div class="chat_wrapper"><div class="chat_box">'),y=Object(o.h)('<div class="msg_own_wrapper"><div class="msg_own">'),k=Object(o.h)('<div class="msg">'),w=Object(o.h)('<div class="msg_user"><span class="msg_user"> <span style="font-size:small;">From: '),_=Object(o.h)('<div class="send_msg"><input type="text" class="send_msg_box"><div class="send_msg_btn">'),[x,S]=Object(r.j)(0),[$,C]=Object(r.j)(null),[E,L]=Object(r.j)(!1);const[N,T]=Object(r.j)();function U(){let e;return Object(r.f)(()=>{!function(e){if(!e)return;e.scrollTop=e.scrollHeight}(e),$()}),(()=>{const t=m(),n=t.firstChild,s=e;return"function"===typeof s?Object(o.i)(s,n):e=n,Object(o.d)(n,Object(r.e)(r.b,{get each(){return function(e){const t=sessionStorage.getItem("chat_history");let n=t?JSON.parse(t):[];return e&&n.push(e),sessionStorage.setItem("chat_history",JSON.stringify(n)),n}($())},children:e=>{const t=Object(r.e)(z,{get msg_sender(){return e.user},get msg(){return e.message}});return T(e.user),t}})),Object(o.d)(t,Object(r.e)(W,{}),null),Object(r.h)(e=>Object(o.g)(t,E()?"":"display:none;",e)),t})()}function z(e){var t;return e.msg_sender==(null===(t=nt())||void 0===t?void 0:t.email)?(()=>{const t=y(),n=t.firstChild;return Object(o.d)(n,()=>e.msg),t})():N()==e.msg_sender?(()=>{const t=k();return Object(o.d)(t,()=>e.msg),t})():[(()=>{const t=w(),n=t.firstChild;n.firstChild.nextSibling;return Object(o.d)(n,()=>e.msg_sender,null),t})(),(()=>{const t=k();return Object(o.d)(t,()=>e.msg),t})()]}function W(){let e;const t=()=>{e.value&&function(e){var t;if(!nt())return;null===(t=rt())||void 0===t||t.send(e)}(e.value),e.value=null};return(()=>{const n=_(),r=n.firstChild,s=r.nextSibling;n.addEventListener("keypress",e=>{"Enter"==e.key&&t()});const i=e;return"function"===typeof i?Object(o.i)(i,r):e=r,s.$$click=t,n})()}function A(e){console.log("Building chat socket on "+e);let t=j("/chat/"+e,e=>{console.log("chat message",e);try{C(JSON.parse(e))}catch(t){console.log(t)}},e=>{st(null)});st(t)}Object(o.c)(["click"]);const I=Object(o.h)('<div class="rocket-container"><div class="structure"><svg height="352" id="rocket-svg" version="1.1" viewbox="0 0 59.266662 93.133333" width="224" xmlns="http://www.w3.org/2000/svg"><g id="layer2" transform="translate(-33.866666,-33.866666)"><path d="m 296,336 a 8.0000078,8.0000078 0 0 0 -8,8 v 80 a 8.0000078,7.9999501 0 0 0 1.16406,4.14062 l -0.22461,0.11329 49.32227,49.32031 0.0781,0.0801 0.004,-0.004 A 7.9999934,8.0000655 0 0 0 344,480 a 7.9999934,8.0000655 0 0 0 8,-8 v -80 a 7.9999934,7.9998924 0 0 0 -2.34961,-5.65625 l 0.004,-0.004 -48.00391,-48.00195 -0.004,0.002 A 8.0000078,8.0000078 0 0 0 296,336 Z" id="right-wing" style="opacity:1;fill-opacity:1;stroke:none;stroke-width:0.99999994;stroke-linecap:round;stroke-linejoin:bevel;stroke-miterlimit:4;stroke-dasharray:none;stroke-dashoffset:0;stroke-opacity:1" transform="scale(0.26458333)"></path><path d="m 184,336 a 8.0000006,8.0000078 0 0 0 -5.65234,2.3457 l -0.004,-0.002 -47.91797,47.91797 -0.082,0.082 0.004,0.002 A 8.0000078,7.9998924 0 0 0 128,392 v 80 a 8.0000078,8.0000655 0 0 0 8,8 8.0000078,8.0000655 0 0 0 5.65625,-2.34961 l 0.004,0.004 49.40039,-49.40039 -0.22657,-0.11329 A 8.0000006,7.9999501 0 0 0 192,424 v -80 a 8.0000006,8.0000078 0 0 0 -8,-8 z" id="left-wing" style="opacity:1;fill-opacity:1;stroke:none;stroke-width:0.99999994;stroke-linecap:round;stroke-linejoin:bevel;stroke-miterlimit:4;stroke-dasharray:none;stroke-dashoffset:0;stroke-opacity:1" transform="scale(0.26458333)"></path><path d="M 239.96875,128 A 111.99996,124.13082 0 0 0 176,240 l 16,200 a 8.0000006,8.0000655 0 0 0 8,8 h 80 a 8.0000078,8.0000655 0 0 0 8,-8 L 304,240 A 111.99996,124.13082 0 0 0 239.96875,128 Z" id="rocket-main-part" style="opacity:1;fill-opacity:1;stroke:none;stroke-width:0.99999994;stroke-linecap:round;stroke-linejoin:bevel;stroke-miterlimit:4;stroke-dasharray:none;stroke-dashoffset:0;stroke-opacity:1" transform="scale(0.26458333)"></path><path d="m 239.96875,128 a 111.99996,124.13082 0 0 0 -47.77344,48 h 95.51953 a 111.99996,124.13082 0 0 0 -47.74609,-48 z" id="nose" style="opacity:1;fill-opacity:1;stroke:none;stroke-width:0.99999994;stroke-linecap:round;stroke-linejoin:bevel;stroke-miterlimit:4;stroke-dasharray:none;stroke-dashoffset:0;stroke-opacity:1" transform="scale(0.26458333)"></path><ellipse cx="63.5" cy="59.266663" id="window-stroke" rx="7.4083333" ry="7.4083328" style="opacity:1;fill-opacity:1;stroke:none;stroke-width:0.26458332;stroke-linecap:round;stroke-linejoin:bevel;stroke-miterlimit:4;stroke-dasharray:none;stroke-dashoffset:0;stroke-opacity:1"></ellipse><ellipse cx="63.499996" cy="59.266666" id="window-inner" rx="6.3499975" ry="6.3500061" style="opacity:1;fill-opacity:1;stroke:none;stroke-width:0.26458332;stroke-linecap:round;stroke-linejoin:bevel;stroke-miterlimit:4;stroke-dasharray:none;stroke-dashoffset:0;stroke-opacity:1"></ellipse><path d="m 240,336 a 7.9999898,8.0000078 0 0 0 -8,8 v 128 a 7.9999898,8.0000078 0 0 0 8,8 7.9999898,8.0000078 0 0 0 8,-8 V 344 a 7.9999898,8.0000078 0 0 0 -8,-8 z" id="middle-wing" style="opacity:1;fill-opacity:1;stroke:none;stroke-width:0.99999994;stroke-linecap:round;stroke-linejoin:bevel;stroke-miterlimit:4;stroke-dasharray:none;stroke-dashoffset:0;stroke-opacity:1" transform="scale(0.26458333)"></path><path d="M 239.96875,128 A 111.99996,124.13082 0 0 0 176,240 l 7.68164,96.01562 a 8.0000006,8.0000078 0 0 0 -5.33398,2.33008 l -0.004,-0.002 -47.91797,47.91797 -0.082,0.082 0.004,0.002 A 8.0000078,7.9998924 0 0 0 128,392 v 80 a 8.0000078,8.0000655 0 0 0 8,8 8.0000078,8.0000655 0 0 0 5.65625,-2.34961 l 0.004,0.004 49.40039,-49.40039 -0.22657,-0.11329 a 8.0000006,7.9999501 0 0 0 0.18946,-0.3496 l 0.0371,0.46289 L 192,440 a 8.0000006,8.0000655 0 0 0 8,8 h 32 v 24 a 7.9999898,8.0000078 0 0 0 8,8 V 336 252 196 128.01758 A 111.99996,124.13082 0 0 0 239.96875,128 Z" id="shadow-layer" style="opacity:0.2;fill:#000000;fill-opacity:1;stroke:none;stroke-width:0.99999994;stroke-linecap:round;stroke-linejoin:bevel;stroke-miterlimit:4;stroke-dasharray:none;stroke-dashoffset:0;stroke-opacity:1" transform="scale(0.26458333)"></div><div class="smoke"><span></span><span></span><span></span><span></span><span></span><span></span><span></span><span></span><span></span><span>'),J=Object(o.h)("<h1>"),P=Object(o.h)('<button class="std_btn"> Cancel '),[q,B]=Object(r.j)(!1),[M,D]=Object(r.j)(),[F,G]=Object(r.j)();function Q(e,t){D(e),B(!0),setTimeout(()=>{B(!1)},300),G(t)}function R(){console.log("Finishing loading!"),D(),G()}function V(){return[Object(r.g)((()=>{const e=Object(r.g)(()=>!!M());return()=>e()?(()=>{const e=J();return Object(o.d)(e,M),e})():[]})()),Object(r.g)((()=>{const e=Object(r.g)(()=>!!F());return()=>e()?(()=>{var e;const t=P();return Object(o.a)(t,"click",null===(e=F())||void 0===e?void 0:e.cb,!0),t})():[]})()),I()]}function H(e){if(e.activeMatch){let t=j("/quoridor/events/"+e.activeMatch,e=>{try{const t=JSON.parse(e);Xe(t)}catch(t){l("Failed to read message from server! Try reloading the page."),console.log(t)}});tt(t),A(e.activeMatch)}else l("Unable to create match, please try again later!")}function Z(e,t){fetch("/quoridor/que/join/"+e).then(e=>{e.ok?e.json().then(H):l("Unable to join game!")}).catch(alert).finally(()=>{t&&t()})}function K(){fetch("/quoridor/que").then(e=>{e.ok?e.json().then(ct):l("Unable to retrive QUE!")})}function Y(){!function(){let e=arguments.length>0&&void 0!==arguments[0]?arguments[0]:g,t=e+"=",n=document.cookie.split(";");for(let o=0;o<n.length;o++){let e=n[o];for(;" "==e.charAt(0);)e=e.substring(1);if(0==e.indexOf(t))return e.substring(t.length,e.length)}}()?ot(null):fetch("/auth/context/").then(e=>X(e)).catch(console.log)}function X(e){if(!e.ok){if(e.status>=500)return void l("Server error please try agian in few minutes!");if(404==e.status)return void l("User not found! Check your login!");if(403==e.status)return void l("Incorrect credentials!")}e.json().then(e=>function(e){if(console.log(e),"AlreadyTaken"===e)return void l("This username is already in use!");if("UnsupportedDataType"in e)return void l(e.UnsupportedDataType);ot(e),p(e.authToken),e.activeMatch&&(Q(),p(e.authToken),Z(e.activeMatch,R))}(e))}Object(o.c)(["click"]);const ee=Object(o.h)('<div class="systen_msg"><p style="color:gray;">Username: <i style="color:red;font-size:large;"></i></p><hr><p style="color:gray;"><i>Press Esc to exit.'),te=Object(o.h)('<div class="covering-panel">'),[ne,oe]=Object(r.j)(!1);function re(e){if("GUEST"==e.user.username)return l("User profile does not exist!\nPlease login and try again!"),void oe(!1);function t(e){"Escape"===e.key&&oe(!1)}Object(r.o)(()=>{document.addEventListener("keydown",t)}),Object(r.n)(()=>{document.removeEventListener("keydown",t)});const[n,s]=Object(r.j)();return fetch("/auth/stats",{method:"get"}).then(e=>{e.ok&&e.json().then(e=>{s(e),console.log(n())})}).catch(e=>{console.log(e),s()}),[(()=>{const t=ee(),s=t.firstChild,i=s.firstChild.nextSibling,l=s.nextSibling.nextSibling;return Object(o.d)(i,()=>e.user.username),Object(o.d)(t,Object(r.e)(O,{get stats(){return n()}}),l),Object(r.h)(e=>Object(o.g)(t,c()?"z-index: 0 !important;":"",e)),t})(),te()]}const se=Object(o.h)("<button>"),ie=Object(o.h)('<nav><div class="nav_block"><div class="box"></div><div class="box"></div></div><div class="image_nav"></div><div class="nav_block"><div class="box"></div><div class="box">');function ce(e){return(()=>{const t=se();return t.$$click=()=>e.click(),Object(o.d)(t,()=>e.text),Object(r.h)(n=>{const r=e.style,s=e.class?e.class:"std_btn",i=e.disabled;return n._v$=Object(o.g)(t,r,n._v$),s!==n._v$2&&Object(o.b)(t,n._v$2=s),i!==n._v$3&&(t.disabled=n._v$3=i),n},{_v$:void 0,_v$2:void 0,_v$3:void 0}),t})()}function le(e){const t=nt();return(()=>{const n=ie(),s=n.firstChild,i=s.firstChild,c=i.nextSibling,l=s.nextSibling.nextSibling.firstChild,a=l.nextSibling;return Object(o.d)(i,t?Object(r.e)(ce,{get text(){return t.username},click:()=>oe(!0)}):[]),Object(o.d)(c,(()=>{const t=Object(r.g)(()=>!e.left);return()=>t()?[]:Object(r.e)(ce,Object(r.m)(()=>e.left))})()),Object(o.d)(l,t?Object(r.e)(ce,{text:"Logout",click:()=>{fetch("/auth/logout").then(e=>{ot(null),tt(null),Xe(null),st(null)}).catch(console.log)}}):[]),Object(o.d)(a,(()=>{const t=Object(r.g)(()=>!e.right);return()=>t()?[]:Object(r.e)(ce,Object(r.m)(()=>e.right))})()),n})()}Object(o.c)(["click"]);const ae=Object(o.h)('<footer>Powered by Axum <a href="https://docs.rs/axum/latest/axum/" target="_blank" style="text-decoration:none;">\u{1f680}</a> | Frontend <a style="text-decoration:none;" href="https://www.solidjs.com/" target="_blank"><i>Solid-JS</i></a> | Developed by <a class="github" href="https://github.com/Dah-phd" target="_blank">Daniel Nikolaev');function de(){return ae()}function ue(e,t,n){const o=[];for(let r in n.horizontal_walls){let s=n.horizontal_walls[r];s[1]!=t&&s[1]!=t-1||(s[0]==e&&o.push(" bottom_red"),s[0]==e-1&&o.push(" top_red"))}for(let r in n.vertical_walls){let s=n.vertical_walls[r];s[0]!=e&&s[0]!=e-1||(s[1]==t&&o.push(" right_red"),s[1]==t-1&&o.push(" left_red"))}return o.join(" ")}const be=Object(o.h)("<div>"),he=Object(o.h)('<div class="row">'),ve=Object(o.h)('<div class="full_screen_centered"><div class="quoridor"><h1> (<!>)</h1><div></div><h1> (<!>)</h1><div class="box"><button class="std_btn">PlayerMove</button><button class="std_btn">Horizontal Border</button><button class="std_btn">Vertical Border'),fe=[0,1,2,3,4,5,6,7,8],ge={move:function(e,t){return console.log(e,t,"m"),{QuoridorMove:{row:e,col:t}}},hW:function(e,t){return console.log(e,t,"h"),{QuoridorWallH:{row:e,col:t}}},vW:function(e,t){return console.log(e,t,"v"),{QuoridorWallV:{row:e,col:t}}}},[je,pe]=Object(r.j)("move"),Oe=new RegExp("wall_bot|wall_top|wall_left|wall_right");function me(e,t,n){const o=document.getElementById("".concat(e).concat(t));o&&(o.className="".concat(o.className," ").concat(n))}function ye(e){return(()=>{const t=be();return t.addEventListener("mouseleave",()=>{"move"!==je()&&function(e,t){if("move"===je())return;const n=[[e,t],[e,t+1],[e+1,t],[e+1,t+1]];for(const o in n){const e=document.getElementById("".concat(n[o][0]).concat(n[o][1]));e&&(e.className=e.className.replace(Oe,""))}}(e.row,e.column)}),t.addEventListener("mouseenter",()=>{"move"!==je()&&function(e,t,n){const o=je();"hW"===o&&function(e,t,n){if(8==e||8==t)return!1;for(const o in n.vertical_walls){const r=n.vertical_walls[o];if(r[0]==e&&r[1]==t)return!1}for(const o in n.horizontal_walls){const r=n.horizontal_walls[o];if(r[0]==e){if(r[1]==t)return!1;if(r[1]==t+1)return!1;if(r[1]==t-1)return!1}}return!0}(e,t,n.game)&&(me(e,t,"wall_bot"),me(e,t+1,"wall_bot"),me(e+1,t,"wall_top"),me(e+1,t+1,"wall_top")),"vW"===o&&function(e,t,n){if(8==e||8==t)return!1;for(const o in n.horizontal_walls){const r=n.horizontal_walls[o];if(r[0]==e&&r[1]==t)return!1}for(const o in n.vertical_walls){const r=n.vertical_walls[o];if(r[1]==t){if(r[0]==e)return!1;if(r[0]==e+1)return!1;if(r[0]==e-1)return!1}}return!0}(e,t,n.game)&&(me(e,t,"wall_right"),me(e+1,t,"wall_right"),me(e,t+1,"wall_left"),me(e+1,t+1,"wall_left"))}(e.row,e.column,e.session)}),t.$$click=()=>{return t=e.row,n=e.column,o=e.session,r=e.ws,s=e.user,i=ge[je()],void(r.readyState===WebSocket.OPEN&&s&&o.current==s.email&&r.send(JSON.stringify(i(t,n))));var t,n,o,r,s,i},Object(r.h)(n=>{const r=function(e,t,n){const o=[ue(e,t,n.game)];return"move"==je()&&o.push(function(e,t,n){const o=n.up_player==n.current?n.game.up_player:n.game.down_player;return o[0]==e&&o[1]==t?"player_move_blocked":o&&function(e,t,n){return e==n[0]&&(t==n[1]-1||t==n[1]+1)||t==n[1]&&(e==n[0]-1||e==n[0]+1)}(e,t,o)?function(e,t,n,o){if(e==n[0]){const r=t>n[1]?n[1]:t;for(let t in o.vertical_walls){const n=o.vertical_walls[t];if(r==n[1]&&(e==n[0]||e-1==n[0]))return!0}}if(t==n[1]){const r=e>n[0]?n[0]:e;for(let e in o.horizontal_walls){const n=o.horizontal_walls[e];if(r==n[0]&&(t==n[1]||t-1==n[1]))return!0}}return!1}(e,t,o,n.game)?"player_move_blocked":"player_move":""}(e,t,n)),"tile"+o.join(" ")+function(e,t,n){let o=" ";return n.up_player[0]==e&&n.up_player[1]==t&&(o+="\u{1f980}"),n.down_player[0]==e&&n.down_player[1]==t&&(o+="\u{1f40d}"),o}(e,t,n.game)}(e.row,e.column,e.session),s="".concat(e.row).concat(e.column);return r!==n._v$&&Object(o.b)(t,n._v$=r),s!==n._v$2&&Object(o.f)(t,"id",n._v$2=s),n},{_v$:void 0,_v$2:void 0}),t})()}function ke(e){return(()=>{const t=he();return Object(o.d)(t,Object(r.e)(r.b,{each:fe,children:t=>Object(r.e)(ye,Object(r.m)({column:t},e))})),t})()}function we(e){if(console.log("game session: ",e.session),e.session)return(()=>{const n=ve(),s=n.firstChild.firstChild,i=s.firstChild,c=i.nextSibling,l=(c.nextSibling,s.nextSibling),a=l.nextSibling,d=a.firstChild,u=d.nextSibling,b=(u.nextSibling,a.nextSibling.firstChild),h=b.nextSibling,v=h.nextSibling;return Object(o.d)(s,()=>e.session.up_player,i),Object(o.d)(s,()=>e.session.game.up_player_free_walls,c),Object(o.d)(l,Object(r.e)(r.b,{each:fe,children:t=>Object(r.e)(ke,{row:t,get session(){return e.session},get ws(){return e.ws},get user(){return e.user}})})),Object(o.d)(a,()=>e.session.down_player,d),Object(o.d)(a,()=>e.session.game.down_player_free_walls,u),b.$$click=()=>pe("move"),h.$$click=()=>pe("hW"),v.$$click=()=>pe("vW"),Object(r.h)(r=>{const i=E()?"display:none;":"",c=t(e.session.up_player),d=e.session.current==e.user.email?"border: solid 10px red;":"border: solid 10px white;",u=t(e.session.down_player),f="move"===je()?"color:red;":"",g="hW"===je()?"color:red;":"",j="vW"===je()?"color:red;":"";return r._v$3=Object(o.g)(n,i,r._v$3),r._v$4=Object(o.g)(s,c,r._v$4),r._v$5=Object(o.g)(l,d,r._v$5),r._v$6=Object(o.g)(a,u,r._v$6),r._v$7=Object(o.g)(b,f,r._v$7),r._v$8=Object(o.g)(h,g,r._v$8),r._v$9=Object(o.g)(v,j,r._v$9),r},{_v$3:void 0,_v$4:void 0,_v$5:void 0,_v$6:void 0,_v$7:void 0,_v$8:void 0,_v$9:void 0}),n})();function t(t){var n;return(null===(n=e.session)||void 0===n?void 0:n.current)==t?"color:red":""}}Object(o.c)(["click"]);const _e=Object(o.h)('<div style="flex-basis:50%;"><h1>Sign In:</h1><div><div><input type="text" placeholder="Email"></div><div><input type="password" placeholder="Password"></div><br><button class="std_btn">Login'),xe=Object(o.h)('<div style="flex-basis:50%;"><h1>Sign In as Guest</h1><div><div><input type="text" placeholder="Username"></div><br><button class="std_btn">Register as Guest'),Se=Object(o.h)('<div style="flex-basis:50%;"><h1>Create New Account:</h1><div><div><input type="text" placeholder="Username"></div><div><input type="password" placeholder="Password"></div><div><input type="password" placeholder="Repeat password"></div><div><input type="email" placeholder="Email"></div><br><button class="std_btn">Register User');function $e(){let e,t;function n(){Q("Loading ..."),function(e,t,n){fetch("/auth/login",{method:"post",headers:{"Content-Type":"application/json"},body:JSON.stringify({email:e,password:t})}).then(e=>X(e)).catch(console.log).finally(()=>{n&&n()})}(e.value,t.value,R),t.value=null}return(()=>{const r=_e(),s=r.firstChild.nextSibling.firstChild,i=s.firstChild,c=s.nextSibling,l=c.firstChild,a=c.nextSibling.nextSibling;r.addEventListener("keypress",e=>{"Enter"==e.key&&n()});const d=e;"function"===typeof d?Object(o.i)(d,i):e=i;const u=t;return"function"===typeof u?Object(o.i)(u,l):t=l,a.$$click=n,r})()}function Ce(){let e;function t(){Q("Loading ..."),function(e,t){fetch("/auth/guest_login",{method:"post",headers:{"Content-Type":"application/json"},body:JSON.stringify({username:e})}).then(e=>X(e)).catch(console.log).finally(()=>{t&&t()})}(e.value,R),e.value=null}return(()=>{const n=xe(),r=n.firstChild.nextSibling.firstChild,s=r.firstChild,i=r.nextSibling.nextSibling;n.addEventListener("keypress",e=>{"Enter"==e.key&&t()});const c=e;return"function"===typeof c?Object(o.i)(c,s):e=s,i.$$click=t,n})()}function Ee(){let e,t,n,r;function s(){if(t.value!=n.value)return l("Passwords do not match!");Q("Loading ..."),function(e,t,n,o){t.length>72?l("Password too long!"):fetch("/auth/register",{method:"post",headers:{"Content-Type":"application/json"},body:JSON.stringify({username:e,password:t,email:n})}).then(e=>X(e)).catch(console.log).finally(()=>{o&&o()})}(e.value,t.value,r.value,R),[e,t,n,r].forEach(e=>e.value=null)}return l("Your email will be visible to other players as identifier!!!"),(()=>{const i=Se(),c=i.firstChild.nextSibling.firstChild,l=c.firstChild,a=c.nextSibling,d=a.firstChild,u=a.nextSibling,b=u.firstChild,h=u.nextSibling,v=h.firstChild,f=h.nextSibling.nextSibling;i.addEventListener("keypress",e=>{"Enter"==e.key&&s()});const g=e;"function"===typeof g?Object(o.i)(g,l):e=l;const j=t;"function"===typeof j?Object(o.i)(j,d):t=d;const p=n;"function"===typeof p?Object(o.i)(p,b):n=b;const O=r;return"function"===typeof O?Object(o.i)(O,v):r=v,f.$$click=s,i})()}Object(o.c)(["click"]);const Le=Object(o.h)("<div><h3>LeaderBoard</h3><hr>"),Ne=Object(o.h)('<div class="lobbies">'),Te=Object(o.h)('<div class="lobby_struct">');function Ue(){const e=sessionStorage.getItem("lobbyInterval");window.clearInterval(Number(e))}function ze(){const[e,t]=Object(r.j)([]);return fetch("/leaderboard").then(e=>{e.ok&&e.json().then(e=>{t(e),console.log(e)})}).catch(e=>{console.log(e)}),(()=>{const t=Le();t.firstChild.nextSibling;return Object(o.d)(t,Object(r.e)(r.b,{get each(){return e()},children:e=>Object(r.e)(O,{stats:e})}),null),t})()}function We(){return sessionStorage.setItem("lobbyInterval",window.setInterval(()=>{K()},5e3).toString()),Object(r.n)(Ue),K(),[Object(r.e)(ze,{}),(()=>{const e=Ne();return Object(o.d)(e,Object(r.e)(r.b,{get each(){return it()},children:e=>(()=>{const t=Te();return t.$$click=()=>{Q(),Z(e,R)},Object(o.d)(t,e),t})()})),e})()]}Object(o.c)(["click"]);const Ae=Object(o.h)('<div class="full_screen_centered"><div class="form_container"> '),Ie=Object(o.h)('<div class="covering-panel"><div class="spin">'),Je=Object(o.h)("<h1>Looking for opponent ..."),Pe=Object(o.h)("<hr>"),qe=Object(o.h)("<h3>Press Esc to cancel"),Be=Object(o.h)('<div class="full_screen_centered">');function Me(){const[e,t]=Object(r.j)(!0);return[Object(r.e)(le,{left:{text:"Sign In",click:()=>{t(!0)}},right:{text:"Create New Account",click:()=>{t(!1)}}}),(()=>{const t=Ae(),n=t.firstChild,s=n.firstChild;return Object(o.d)(n,(()=>{const t=Object(r.g)(()=>!!e());return()=>t()?Object(r.e)($e,{}):Object(r.e)(Ee,{})})(),s),Object(o.d)(n,Object(r.e)(Ce,{}),null),t})(),Object(r.e)(de,{})]}const[De,Fe]=Object(r.j)(!1);function Ge(){function e(){function e(e){e.key}return Object(r.o)(()=>{document.addEventListener("keydown",e)}),Object(r.n)(()=>{document.removeEventListener("keydown",e)}),[Ie(),Object(r.e)(le,{}),Je(),Pe(),qe()]}return p(nt().authToken),Object(r.e)(r.d,{get children(){return[Object(r.e)(r.c,{get when(){return De()},get children(){return Object(r.e)(e,{})}}),Object(r.e)(r.c,{get when(){return!De()},get children(){return[Object(r.e)(le,{left:{text:"Game VS CPU",click:()=>{fetch("/quoridor/solo").then(e=>{e.ok?e.json().then(H):l("Unable to create solo game!")}).catch(e=>{alert(e)}).finally(R)}},right:{text:"Create Lobby",click:()=>{!function(){var e;if(null!==(e=et())&&void 0!==e&&e.OPEN)return void l("Already connected to game, refresh to reconnect!");const t=j("/quoridor/que/host",e=>{console.log("event on builder",e),t.close();let n=j("/quoridor/events/"+e,e=>{try{const t=JSON.parse(e);Xe(t)}catch(t){l("Failed to read message from server! Try reloading the page.")}});tt(n),A(e)},e=>{R()});console.log("setting cancel!!"),Q("Looking for game ...",{cb:()=>t.close()})}()}}}),(()=>{const e=Be();return Object(o.d)(e,Object(r.e)(We,{})),e})(),Object(r.e)(de,{})]}})]}})}function Qe(e){const[t,n]=Object(r.j)("Concede"),s=()=>{var n;"Concede"==t()?(n=e.ws).readyState===WebSocket.OPEN&&n.send(JSON.stringify("Concede")):(e.ws.close(),tt(null),st(null))};return Object(r.f)(()=>{e.session.winner?(l("Winner is ".concat(e.session.winner)),n("Back to Lobbies")):n("Concede")}),[Object(r.e)(le,{get right(){return{text:t(),style:"color:red;",click:s}},get left(){return{text:E()?"Back to Game":"Open Chat ".concat(x()?x():""),style:x()?"color: red;":"",click:()=>{L(!E()),E()&&S(0)}}}}),Object(r.e)(we,{get ws(){return e.ws},get session(){return e.session},get user(){return nt()}}),(()=>{const e=Be();return Object(o.d)(e,Object(r.e)(U,{})),e})()]}const Re=Object(o.h)('<div class="tetrominos"><div class="tetromino box1"></div><div class="tetromino box2"></div><div class="tetromino box3"></div><div class="tetromino box4">');function Ve(){return Re()}const[He,Ze]=Object(r.j)(!1);const Ke=navigator.userAgent.toLowerCase().match(/mobile/i),[Ye,Xe]=Object(r.j)(null),[et,tt]=Object(r.j)(null),[nt,ot]=Object(r.j)(null),[rt,st]=Object(r.j)(null),[it,ct]=Object(r.j)([]);var lt=function(){return Y(),Ke&&l("Quoridor is not yet optimized to be used on mobile device. Use at your own risk!"),[Object(r.e)(r.d,{get children(){return[Object(r.e)(r.c,{get when(){return q()||M()},get children(){return Object(r.e)(V,{})}}),Object(r.e)(r.c,{get when(){return!nt()},get children(){return[Object(r.e)(Ve,{}),Object(r.e)(Me,{})]}}),Object(r.e)(r.c,{get when(){return!et()},get children(){return Object(r.e)(Ge,{})}}),Object(r.e)(r.c,{get when(){return Object(r.g)(()=>!!et())()&&Ye()},get children(){return Object(r.e)(Qe,{get ws(){return et()},get session(){return Ye()}})}})]}}),Object(r.g)((()=>{const e=Object(r.g)(()=>!!ne());return()=>e()?Object(r.e)(re,{get user(){return nt()}}):[]})()),Object(r.g)((()=>{const e=Object(r.g)(()=>!!c());return()=>e()?Object(r.e)(b,{}):[]})())]};Boolean("localhost"===window.location.hostname||"[::1]"===window.location.hostname||window.location.hostname.match(/^127(?:\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}$/));Object(o.e)(lt,document.getElementById("root")),"serviceWorker"in navigator&&navigator.serviceWorker.ready.then(e=>{e.unregister()})}],[[3,1,2]]]);