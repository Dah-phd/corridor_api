(window.webpackJsonp=window.webpackJsonp||[]).push([[0],[,,function(e,t,n){},function(e,t,n){"use strict";n.r(t);var o=n(0),c=(n(2),n(1));function r(e){s("gamertag",e)}function i(){return l("gamertag")}function s(e,t){let n=arguments.length>2&&void 0!==arguments[2]?arguments[2]:30,o="";if(n){let e=new Date;e.setTime(e.getTime()+24*n*60*60*1e3),o="; expires="+e.toUTCString()}document.cookie=e+"="+(t||"")+o+"; path=/"}function l(e){let t=new RegExp("(?<=".concat(e,"=)[^;]*")),n=document.cookie.match(t);return n?n[0]:""}const[a,u]=Object(c.f)(v()),[b,d]=Object(c.f)(localStorage.getItem("userType"));function h(e,t){localStorage.setItem("userName",e),u(e);t&&t in["user","guest"]&&(d(t),localStorage.setItem("userType",t))}function v(){if(!i())return null;let e=localStorage.getItem("userName");return e||(fetch("/auth/get_username").then(e=>e.json()).then(e=>{console.log(e),localStorage.setItem("userName",e[0]),r(e[1])}).catch(e=>console.log(e)),e=l("userName")),e}function g(){s("gamertag","",-1),u(null),d(null),location.reload()}const f=Object(o.g)("<button></button>",2),p=Object(o.g)('<nav><div class="nav_block"><div class="box"></div><div class="box"></div></div><div class="image_nav"></div><div class="nav_block"><div class="box"></div><div class="box"></div></div></nav>',16);function m(e){return(()=>{const t=f.cloneNode(!0);return t.$$click=()=>e.click(),Object(o.c)(t,()=>e.text),Object(c.d)(n=>{const c=e.style,r=e.class?e.class:"std_btn";return n._v$=Object(o.f)(t,c,n._v$),r!==n._v$2&&Object(o.a)(t,n._v$2=r),n},{_v$:void 0,_v$2:void 0}),t})()}function j(e){const t=v();return(()=>{const n=p.cloneNode(!0),r=n.firstChild,i=r.firstChild,s=i.nextSibling,l=r.nextSibling.nextSibling.firstChild,a=l.nextSibling;return Object(o.c)(i,t?Object(c.b)(m,{text:t,click:()=>alert("Not Implemented")}):[]),Object(o.c)(s,(()=>{const t=Object(o.d)(()=>!e.left,!0);return()=>t()?[]:Object(c.b)(m,{get text(){return e.left.text},get click(){return e.left.click}})})()),Object(o.c)(l,t?Object(c.b)(m,{text:"Logout",click:()=>g()}):[]),Object(o.c)(a,(()=>{const t=Object(o.d)(()=>!e.right,!0);return()=>t()?[]:Object(c.b)(m,{get text(){return e.right.text},get click(){return e.right.click}})})()),n})()}Object(o.b)(["click"]);const O=Object(o.g)('<div><h1>Sign In:</h1><form><p><input type="text" name="username" placeholder="Username"></p><p><input type="password" name="password" placeholder="Password"></p><p><button class="std_btn" type="submit">Login</button></p></form></div>',16),_=Object(o.g)('<div><h1>Sign In as Guest</h1><form><p><input type="text" name="username" placeholder="Username"></p><p><button class="std_btn">Register as Guest</button></p></form></div>',13),w=Object(o.g)('<div><h1>Create New Account:</h1><form><p><input type="text" name="username" placeholder="Username"></p><p><input type="password" name="password" placeholder="Password"></p><p><input type="password" name="password2" placeholder="Repeat password"></p><p><input type="email" name="email" placeholder="Email"></p><p><button class="std_btn">Register User</button></p></form></div>',22);function y(e){const t=e.target.parentElement.parentElement;if(!t)throw"form not found";return t}function k(e){e.preventDefault();const t=new FormData(y(e));var n,o;n=t.get("username"),o=t.get("password"),fetch("/auth/login",{method:"post",body:JSON.stringify({User:[n,o]})}).then(e=>e.json()).then(e=>{e.Ok&&(r(e.Ok),h(n,"user"))}).catch(console.log)}function S(e){e.preventDefault();const t=new FormData(y(e));!function(e,t,n){if(t.length>72)return"password too long";fetch("/auth/register",{method:"post",body:JSON.stringify({user:e,password:t,email:n})}).then(e=>e.json()).then(t=>{t.Ok&&r(t.Ok),h(e,"user")}).catch(alert)}(t.get("username"),t.get("password"),t.get("email"))}function $(e){e.preventDefault();const t=new FormData(y(e));var n;n=t.get("username"),fetch("/auth/login",{method:"post",body:JSON.stringify({Guest:n})}).then(e=>e.json()).then(e=>{e.Ok&&(r(e.Ok),h(n,"guest"))}).catch(alert)}function x(e){return[(()=>{const t=O.cloneNode(!0);return t.firstChild.nextSibling.firstChild.nextSibling.nextSibling.firstChild.$$click=k,Object(c.d)(n=>Object(o.f)(t,e.show?"display:block;":"display:none;flex-basis: 50%;",n)),t})(),(()=>{const t=_.cloneNode(!0);return t.firstChild.nextSibling.firstChild.nextSibling.firstChild.$$click=$,Object(c.d)(n=>Object(o.f)(t,e.show?"display:block;":"display:none;flex-basis: 50%;",n)),t})()]}function C(e){return(()=>{const t=w.cloneNode(!0),n=t.firstChild.nextSibling;return n.firstChild.nextSibling.nextSibling.nextSibling.nextSibling.firstChild.$$click=S,Object(c.d)(c=>{const r=e.show?"display:block;":"display:none;flex-basis: 50%;",i=e.show?"display:block":"display:none";return c._v$=Object(o.f)(t,r,c._v$),c._v$2=Object(o.f)(n,i,c._v$2),c},{_v$:void 0,_v$2:void 0}),t})()}function N(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}Object(o.b)(["click"]);class I{constructor(e,t){let n=arguments.length>2&&void 0!==arguments[2]?arguments[2]:1;N(this,"status",void 0),N(this,"uri",void 0),N(this,"event_src",void 0),N(this,"callback",void 0),N(this,"retry",void 0),this.uri=e,this.event_src=new EventSource(e),this.callback=t,this.retry=n,this.status=!1,this.connect()}kill(){this.event_src.close()}connect(){this.event_src.addEventListener("message",e=>{this.callback(e,this)}),this.event_src.addEventListener("open",()=>{this.status=!0,console.log("connected to event stream at "+this.uri)}),this.event_src.addEventListener("error",()=>{this.status=!1,this.event_src.close();let e=this.retry;this.retry=Math.min(64,2*this.retry),console.log("connection lost. attempting to reconnect in ".concat(e,"s")),setTimeout(()=>{this.event_src=new EventSource(this.uri),this.connect()},1e3*e)})}}function E(){return localStorage.getItem("active_game")}function W(e,t){localStorage.setItem("active_game",e),U(t),new I("/events/<owner>".replace("<owner>",e),t)}function U(e){const t=E();t&&v()&&fetch("/game_state/<owner>".replace("<owner>",t)).then(e=>e.json()).then(t=>{t?e(t):localStorage.removeItem("active_game")}).catch(console.log)}const D=Object(o.g)('<div class="lobby_struct"> /<!>/</div>',3),J=Object(o.g)('<div class="lobbies"></div>',2);function Q(e){return(()=>{const t=D.cloneNode(!0),n=t.firstChild,c=n.nextSibling;c.nextSibling;return t.$$click=()=>{var t,n;t=e.lobby.owner,n=e.gameSetter,fetch("/join/<owner>".replace("<owner>",t)).then(e=>e.json()).then(e=>{e&&W(e,n)}).catch(console.log)},Object(o.c)(t,()=>e.lobby.match_type,n),Object(o.c)(t,()=>e.lobby.owner,c),t})()}function L(e){const[t,n]=Object(c.f)([]);return window.setInterval(()=>{var e;e=n,fetch("/active_lobbies").then(e=>e.json()).then(t=>{t&&e(t)}).catch(console.log)},3e4),(()=>{const n=J.cloneNode(!0);return Object(o.c)(n,Object(c.b)(c.a,{get each(){return t()},children:t=>Object(c.b)(Q,{lobby:t,get gameSetter(){return e.gameSetter}})})),n})()}Object(o.b)(["click"]);function P(e,t,n){const o=[];for(let c in n.horizontal_walls){let r=n.horizontal_walls[c];r[1]!=t&&r[1]!=t-1||(r[0]==e&&o.push(" bottom_red"),r[0]==e-1&&o.push(" top_red"))}for(let c in n.vertical_walls){let r=n.vertical_walls[c];r[0]!=e&&r[0]!=e-1||(r[1]==t&&o.push(" right_red"),r[1]==t-1&&o.push(" left_red"))}return o.join(" ")}function T(e,t,n){const o=n.up_player==n.current?n.game.up_player:n.game.down_player;return o[0]==e&&o[1]==t?"player_move_blocked":o&&function(e,t,n){return e==n[0]&&(t==n[1]-1||t==n[1]+1)||t==n[1]&&(e==n[0]-1||e==n[0]+1)}(e,t,o)?function(e,t,n,o){if(e==n[0]){const c=t>n[1]?n[1]:t;for(let t in o.vertical_walls){const n=o.vertical_walls[t];if(c==n[1]&&(e==n[0]||e-1==n[0]))return!0}}if(t==n[1]){const c=e>n[0]?n[0]:e;for(let e in o.horizontal_walls){const n=o.horizontal_walls[e];if(c==n[0]&&(t==n[1]||t-1==n[1]))return!0}}return!1}(e,t,o,n.game)?"player_move_blocked":"player_move":void 0}const z=Object(o.g)("<div></div>",2),A=Object(o.g)('<div class="row"></div>',2),B=Object(o.g)('<div class="full_screen_centered"><div class="quoridor"><h1></h1><div></div><h1></h1><div class="box"><button class="std_btn">PlayerMove</button><button class="std_btn">Horizontal Border: </button><button class="std_btn">Vertical Border: </button></div></div></div>',18),G=[0,1,2,3,4,5,6,7,8],R={move:function(e,t,n){return console.log(e,t,"m"),{QuoridorMove:[[e,t],n]}},hW:function(e,t,n){return console.log(e,t,"h"),{QuoridorWallH:[[e,t],n]}},vW:function(e,t,n){return console.log(e,t,"v"),{QuoridorWallV:[[e,t],n]}}},[F,M]=Object(c.f)("move");function V(e){return(()=>{const t=z.cloneNode(!0);return t.$$click=()=>function(e,t,n,o){o(e,t,"test");const c=v(),r=i();if(!c||!r||n.current!=c)return;const s="/move/<owner>".replace("<owner>",n.up_player);fetch(s,{method:"post",headers:{token:r},body:JSON.stringify(o(e,t,c))})}(e.row,e.column,e.session,R[F()]),Object(o.c)(t,()=>{return t=e.row,n=e.column,(o=e.session.game).up_player[0]==t&&o.up_player[1]==n?"U":o.down_player[0]==t&&o.down_player[1]==n?"D":void 0;var t,n,o}),Object(c.d)(()=>{return Object(o.a)(t,"tile"+(n=e.row,c=e.column,r=e.session,[P(n,c,r.game),"move"==F()?T(n,c,r):void 0].join(" ")));var n,c,r}),t})()}function H(e){return(()=>{const t=A.cloneNode(!0);return Object(o.c)(t,Object(c.b)(c.a,{each:G,children:t=>Object(c.b)(V,{column:t,get row(){return e.row},get session(){return e.game}})})),t})()}function q(e){function t(t){return e.session.current==t?"color:red":""}return(()=>{const n=B.cloneNode(!0),r=n.firstChild.firstChild,i=r.nextSibling,s=i.nextSibling,l=s.nextSibling.firstChild,a=l.nextSibling,u=(a.firstChild,a.nextSibling);u.firstChild;return Object(o.c)(r,()=>e.session.up_player),Object(o.c)(i,Object(c.b)(c.a,{each:G,children:t=>Object(c.b)(H,{row:t,get game(){return e.session}})})),Object(o.c)(s,()=>e.session.down_player),l.$$click=()=>M("move"),a.$$click=()=>M("hW"),Object(o.c)(a,()=>e.session.game.up_player_free_walls,null),u.$$click=()=>M("vW"),Object(o.c)(u,()=>e.session.game.down_player_free_walls,null),Object(c.d)(n=>{const c=t(e.session.up_player),i=t(e.session.down_player),b="move"===F()?"color:red;":"",d="hW"===F()?"color:red;":"",h="vW"===F()?"color:red;":"";return n._v$=Object(o.f)(r,c,n._v$),n._v$2=Object(o.f)(s,i,n._v$2),n._v$3=Object(o.f)(l,b,n._v$3),n._v$4=Object(o.f)(a,d,n._v$4),n._v$5=Object(o.f)(u,h,n._v$5),n},{_v$:void 0,_v$2:void 0,_v$3:void 0,_v$4:void 0,_v$5:void 0}),n})()}Object(o.b)(["click"]);const K=Object(o.g)('<div class="full_screen_centered"><div class="form_container"></div></div>',4),X=Object(o.g)('<div class="full_screen_centered"></div>',2);function Y(){const[e,t]=Object(c.f)(!0),[n,r]=Object(c.f)(!1);return[Object(c.b)(j,{left:{text:"Sign In",click:()=>{t(!e()),r(!1)}},right:{text:"Create New Account",click:()=>{r(!n()),t(!1)}}}),(()=>{const t=K.cloneNode(!0),r=t.firstChild;return Object(o.c)(r,Object(c.b)(x,{get show(){return e()}}),null),Object(o.c)(r,Object(c.b)(C,{get show(){return n()}}),null),t})()]}const[Z,ee]=Object(c.f)(null);function te(){return[Object(c.b)(j,{left:{text:"Game VS CPU",click:()=>{var e;e=ee,fetch("/join/<owner>".replace("<owner>","|QCPU|")).then(e=>e.json()).then(t=>{t&&W(t,e)}).catch(console.log)}},right:{text:"Create Lobby",click:()=>{!function(e){const t={owner:v(),game:"Quoridor"};fetch("/create_lobby",{method:"post",body:JSON.stringify(t)}).then(e=>e.json()).then(t=>{t&&new I("/lobby_events/<owner>".replace("<owner>",t),(t,n)=>{console.log(t),t&&t.game_started&&(W(t.game_started,e),n.kill())})}).catch(console.log)}(ee)}}}),(()=>{const e=X.cloneNode(!0);return Object(o.c)(e,Object(c.b)(L,{gameSetter:ee})),e})()]}function ne(){return[Object(c.b)(j,{}),Object(c.b)((function(e){if(e.session)return e.session.ActiveQuoridor?Object(c.b)(q,{get session(){return e.session.ActiveQuoridor}}):[]}),{get session(){return Z()}})]}var oe=function(){return console.log(a()),function(e){const t=localStorage.getItem("active_game");if(t)U(e)}(ee),[Object(o.d)((()=>{const e=Object(o.d)(()=>!a(),!0);return()=>e()?Object(c.b)(Y,{}):[]})()),Object(o.d)((()=>{const e=Object(o.d)(()=>!(!a()||E()),!0);return()=>e()?Object(c.b)(te,{}):[]})()),Object(o.d)((()=>{const e=Object(o.d)(()=>!(!a()||!E()),!0);return()=>e()?Object(c.b)(ne,{}):[]})())]};Boolean("localhost"===window.location.hostname||"[::1]"===window.location.hostname||window.location.hostname.match(/^127(?:\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}$/));Object(o.e)(oe,document.getElementById("root")),"serviceWorker"in navigator&&navigator.serviceWorker.ready.then(e=>{e.unregister()})}],[[3,1,2]]]);