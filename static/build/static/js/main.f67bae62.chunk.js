(window.webpackJsonp=window.webpackJsonp||[]).push([[0],[,,function(e,t,n){},function(e,t,n){"use strict";n.r(t);var o=n(0),c=(n(2),n(1));function r(e){s("gamertag",e)}function s(e,t){let n=arguments.length>2&&void 0!==arguments[2]?arguments[2]:30,o="";if(n){let e=new Date;e.setTime(e.getTime()+24*n*60*60*1e3),o="; expires="+e.toUTCString()}document.cookie=e+"="+(t||"")+o+"; path=/"}function i(e){let t=new RegExp("(?<=".concat(e,"=)[^;]*")),n=document.cookie.match(t);return n?n[0]:""}const[l,a]=Object(c.h)(g()),[u,d]=Object(c.h)(localStorage.getItem("userType"));function b(e,t){localStorage.setItem("userName",e),a(e);t&&t in["user","guest"]&&(d(t),localStorage.setItem("userType",t))}function g(){if(!i("gamertag"))return null;let e=localStorage.getItem("userName");return e||(fetch("/auth/get_username").then(e=>e.json()).then(e=>{console.log(e),localStorage.setItem("userName",e[0]),r(e[1])}).catch(e=>console.log(e)),e=i("userName")),e}function h(e){const t=e.target.parentElement.parentElement;if(!t)throw"form not found";return t}const v=Object(o.h)("<button></button>",2),m=Object(o.h)('<nav><div class="nav_block"><div class="box"></div><div class="box"></div></div><div class="image_nav"></div><div class="nav_block"><div class="box"></div><div class="box"></div></div></nav>',16);function f(e){return(()=>{const t=v.cloneNode(!0);return t.$$click=()=>e.click(),Object(o.c)(t,()=>e.text),Object(c.f)(n=>{const c=e.style,r=e.class?e.class:"std_btn";return n._v$=Object(o.g)(t,c,n._v$),r!==n._v$2&&Object(o.a)(t,n._v$2=r),n},{_v$:void 0,_v$2:void 0}),t})()}function p(e){const t=g();return(()=>{const n=m.cloneNode(!0),r=n.firstChild,i=r.firstChild,l=i.nextSibling,u=r.nextSibling.nextSibling.firstChild,b=u.nextSibling;return Object(o.c)(i,t?Object(c.d)(f,{text:t,click:()=>alert("Not Implemented")}):[]),Object(o.c)(l,(()=>{const t=Object(o.d)(()=>!e.left,!0);return()=>t()?[]:Object(c.d)(f,{get text(){return e.left.text},get click(){return e.left.click},get style(){return e.left.style},get class(){return e.left.class}})})()),Object(o.c)(u,t?Object(c.d)(f,{text:"Logout",click:()=>(s("gamertag","",-1),a(null),d(null),localStorage.clear(),void location.reload())}):[]),Object(o.c)(b,(()=>{const t=Object(o.d)(()=>!e.right,!0);return()=>t()?[]:Object(c.d)(f,{get text(){return e.right.text},get click(){return e.right.click},get style(){return e.right.style},get class(){return e.right.class}})})()),n})()}Object(o.b)(["click"]);const O=Object(o.h)('<div><h1>Sign In:</h1><form><p><input type="text" name="username" placeholder="Username"></p><p><input type="password" name="password" placeholder="Password"></p><p><button class="std_btn" type="submit">Login</button></p></form></div>',16),j=Object(o.h)('<div><h1>Sign In as Guest</h1><form><p><input type="text" name="username" placeholder="Username"></p><p><button class="std_btn">Register as Guest</button></p></form></div>',13),_=Object(o.h)('<div><h1>Create New Account:</h1><form><p><input type="text" name="username" placeholder="Username"></p><p><input type="password" name="password" placeholder="Password"></p><p><input type="password" name="password2" placeholder="Repeat password"></p><p><input type="email" name="email" placeholder="Email"></p><p><button class="std_btn">Register User</button></p></form></div>',22);function w(e){e.preventDefault();const t=new FormData(h(e));var n,o;n=t.get("username"),o=t.get("password"),fetch("/auth/login",{method:"post",body:JSON.stringify({User:[n,o]})}).then(e=>e.json()).then(e=>{e.Ok&&(r(e.Ok),b(n,"user"))}).catch(console.log)}function y(e){e.preventDefault();const t=new FormData(h(e));!function(e,t,n){if(t.length>72)return"password too long";fetch("/auth/register",{method:"post",body:JSON.stringify({user:e,password:t,email:n})}).then(e=>e.json()).then(t=>{t.Ok&&r(t.Ok),b(e,"user")}).catch(alert)}(t.get("username"),t.get("password"),t.get("email"))}function S(e){e.preventDefault();const t=new FormData(h(e));var n;n=t.get("username"),fetch("/auth/login",{method:"post",body:JSON.stringify({Guest:n})}).then(e=>e.json()).then(e=>{e.Ok&&(r(e.Ok),b(n,"guest"))}).catch(alert)}function k(e){return[(()=>{const t=O.cloneNode(!0);return t.firstChild.nextSibling.firstChild.nextSibling.nextSibling.firstChild.$$click=w,Object(c.f)(n=>Object(o.g)(t,e.show?"display:block; flex-basis: 50%;":"display:none;",n)),t})(),(()=>{const t=j.cloneNode(!0);return t.firstChild.nextSibling.firstChild.nextSibling.firstChild.$$click=S,Object(c.f)(n=>Object(o.g)(t,e.show?"display:block; flex-basis: 50%;":"display:none;",n)),t})()]}function x(e){return(()=>{const t=_.cloneNode(!0),n=t.firstChild.nextSibling;return n.firstChild.nextSibling.nextSibling.nextSibling.nextSibling.firstChild.$$click=y,Object(c.f)(c=>{const r=e.show?"display:block; flex-basis: 100%;":"display:none;",s=e.show?"display:block":"display:none";return c._v$=Object(o.g)(t,r,c._v$),c._v$2=Object(o.g)(n,s,c._v$2),c},{_v$:void 0,_v$2:void 0}),t})()}function $(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}Object(o.b)(["click"]);class N{constructor(e,t){let n=arguments.length>2&&void 0!==arguments[2]?arguments[2]:1;$(this,"status",void 0),$(this,"uri",void 0),$(this,"event_src",void 0),$(this,"callback",void 0),$(this,"retry",void 0),this.uri=e,this.event_src=new EventSource(e),this.callback=t,this.retry=n,this.status=!1,this.connect()}kill(){this.event_src.close()}connect(){this.event_src.addEventListener("message",e=>{this.callback(JSON.parse(e.data),this)}),this.event_src.addEventListener("open",()=>{this.status=!0,console.log("connected to event stream at "+this.uri)}),this.event_src.addEventListener("error",()=>{this.status=!1,this.event_src.close();let e=this.retry;this.retry=Math.min(64,2*this.retry),console.log("connection lost. attempting to reconnect in ".concat(e,"s")),setTimeout(()=>{this.event_src=new EventSource(this.uri),this.connect()},1e3*e)})}}const I=Object(o.h)('<div class="chat_box"></div>',2),C=Object(o.h)('<p class="msg_own"> <span class="msg_own"></span></p>',4),E=Object(o.h)('<p class="msg"><span class="msg"></span></p>',4),L=Object(o.h)('<p class="msg_user"><span class="msg_user"> <span style="font-size:small;">From: </span></span></p>',6),J=Object(o.h)('<form class="form_container"><input type="text" name="message" class="message_box"><p><button class="std_btn message_box_btn" type="submit">Send</button></p></form>',7),[Q,W]=Object(c.h)(0),[U,D]=Object(c.h)(null),[z,A]=Object(c.h)(!1);function B(e){const t=localStorage.getItem(K);if(!t)return;const n="chat_".concat(JSON.stringify(t)),o=function(e){const t=sessionStorage.getItem(e);return t?JSON.parse(t):[]}(n);return e&&o.push(e),sessionStorage.setItem(n,JSON.stringify(o)),o}const[P,T]=Object(c.h)("");function R(e){return(()=>{const t=I.cloneNode(!0);return Object(o.c)(t,Object(c.d)(c.a,{get each(){return B(e.new_msg)},children:e=>{const t=Object(c.d)(G,{get msg_sender(){return e.player},get msg(){return e.msg}});return T(e.player),t}}),null),Object(o.c)(t,Object(c.d)(F,{}),null),Object(c.f)(e=>Object(o.g)(t,z()?"":"display:none;",e)),t})()}function G(e){return l()==e.msg_sender?(()=>{const t=C.cloneNode(!0),n=t.firstChild.nextSibling;return Object(o.c)(n,()=>e.msg),t})():P()==e.msg_sender?(()=>{const t=E.cloneNode(!0),n=t.firstChild;return Object(o.c)(n,()=>e.msg),t})():[(()=>{const t=L.cloneNode(!0),n=t.firstChild;n.firstChild.nextSibling;return Object(o.c)(n,()=>e.msg_sender,null),t})(),(()=>{const t=E.cloneNode(!0),n=t.firstChild;return Object(o.c)(n,()=>e.msg),t})()]}function F(){return(()=>{const e=J.cloneNode(!0);return e.firstChild.nextSibling.firstChild.$$click=e=>{e.preventDefault();const t=h(e);!function(e){if(!e)return;e.scrollTop=e.scrollHeight}(t.parentElement);const n=new FormData(t);var o;t.reset(),o=n.get("message"),fetch("/chat/sender",{method:"post",body:JSON.stringify({id:{MatchID:localStorage.getItem("activeChat")},msg:o,player:l()})})},e})()}Object(o.b)(["click"]);function M(){const e=localStorage.getItem(K);e&&(localStorage.setItem("activeChat",e),new N("/game_chat/<owner>".replace("<owner>",e),q))}const[V,H]=Object(c.h)(null);function q(e,t){D(e),z()||W(Q()+1),H(t)}const K="active_game";function X(e,t){localStorage.setItem(K,e),M(),ee(t),new N("/game_events/<owner>".replace("<owner>",e),(e,n)=>{Z(e,n,t)})}function Y(){const e=sessionStorage.getItem("stateBackupTimeout");null!=e&&window.clearInterval(Number(e))}function Z(e,t,n){n(e),function(e,t){if(e&&e.ActiveQuoridor&&e.ActiveQuoridor.winner)return function(e,t){var n;alert("GAME OVER!\nWINNER IS ".concat(null===e||void 0===e||null===(n=e.ActiveQuoridor)||void 0===n?void 0:n.winner)),localStorage.removeItem(K),t(null);const o=V();null===o||void 0===o||o.kill(),localStorage.removeItem("activeChat")}(e,t),!0;return!1}(e,n)&&(null===t||void 0===t||t.kill())}function ee(e){const t=localStorage.getItem(K);t&&g()&&fetch("/game_state/<owner>".replace("<owner>",t)).then(e=>e.json()).then(t=>Z(t,void 0,e)).catch(console.log)}function te(e,t,n){const o=[];for(let c in n.horizontal_walls){let r=n.horizontal_walls[c];r[1]!=t&&r[1]!=t-1||(r[0]==e&&o.push(" bottom_red"),r[0]==e-1&&o.push(" top_red"))}for(let c in n.vertical_walls){let r=n.vertical_walls[c];r[0]!=e&&r[0]!=e-1||(r[1]==t&&o.push(" right_red"),r[1]==t-1&&o.push(" left_red"))}return o.join(" ")}function ne(e){fetch("/active_lobbies").then(e=>e.json()).then(t=>{t&&e(t)}).catch(console.log)}function oe(e,t){new N("/lobby_events/<owner>".replace("<owner>",e),(e,n)=>{e&&e.game_started&&(localStorage.removeItem("activeLobby"),Ee(!1),n.kill(),X(e.game_started,t))})}const ce=Object(o.h)('<div class="lobby_struct"> /<hr></div>',3),re=Object(o.h)('<div class="lobbies"></div>',2);function se(){const e=sessionStorage.getItem("lobbyInterval");console.log(e),window.clearInterval(Number(e))}function ie(e){return(()=>{const t=ce.cloneNode(!0),n=t.firstChild,c=n.nextSibling;return t.$$click=()=>{var t,n;t=e.lobby.owner,n=e.gameSetter,fetch("/join/<owner>".replace("<owner>",t)).then(e=>e.json()).then(e=>{e&&(console.log(e),X(e,n))}).catch(console.log)},Object(o.c)(t,()=>e.lobby.match_type,n),Object(o.c)(t,()=>e.lobby.owner,c),t})()}function le(e){const[t,n]=Object(c.h)([]);var r;return r=n,sessionStorage.setItem("lobbyInterval",window.setInterval(()=>{ne(r)},1e4).toString()),Object(c.j)(se),ne(n),(()=>{const n=re.cloneNode(!0);return Object(o.c)(n,Object(c.d)(c.a,{get each(){return t()},children:t=>Object(c.d)(ie,{lobby:t,get gameSetter(){return e.gameSetter}})})),n})()}Object(o.b)(["click"]);const ae=Object(o.h)("<div></div>",2),ue=Object(o.h)('<div class="row"></div>',2),de=Object(o.h)('<div class="full_screen_centered"><div class="quoridor"><h1> (<!>)</h1><div></div><h1> (<!>)</h1><div class="box"><button class="std_btn">PlayerMove</button><button class="std_btn">Horizontal Border</button><button class="std_btn">Vertical Border</button></div></div></div>',20),be=[0,1,2,3,4,5,6,7,8],ge={move:function(e,t,n){return console.log(e,t,"m"),{QuoridorMove:[[e,t],n]}},hW:function(e,t,n){return console.log(e,t,"h"),{QuoridorWallH:[[e,t],n]}},vW:function(e,t,n){return console.log(e,t,"v"),{QuoridorWallV:[[e,t],n]}}},[he,ve]=Object(c.h)("move"),me=new RegExp("wall_bot|wall_top|wall_left|wall_right");function fe(e,t,n){const o=document.getElementById("".concat(e).concat(t));o&&(o.className="".concat(o.className," ").concat(n))}function pe(e){return(()=>{const t=ae.cloneNode(!0);return t.addEventListener("mouseleave",()=>{"move"!==he()&&function(e,t,n){if("move"===he())return;const o=[[e,t],[e,t+1],[e+1,t],[e+1,t+1]];for(const c in o){const e=document.getElementById("".concat(o[c][0]).concat(o[c][1]));e&&(e.className=e.className.replace(me,""))}}(e.row,e.column,e.session)}),t.addEventListener("mouseenter",()=>{"move"!==he()&&function(e,t,n){const o=he();"hW"===o&&function(e,t,n){if(8==e||8==t)return!1;for(const o in n.vertical_walls){const c=n.vertical_walls[o];if(c[0]==e&&c[1]==t)return!1}for(const o in n.horizontal_walls){const c=n.horizontal_walls[o];if(c[0]==e){if(c[1]==t)return!1;if(c[1]==t+1)return!1;if(c[1]==t-1)return!1}}return!0}(e,t,n.game)&&(fe(e,t,"wall_bot"),fe(e,t+1,"wall_bot"),fe(e+1,t,"wall_top"),fe(e+1,t+1,"wall_top")),"vW"===o&&function(e,t,n){if(8==e||8==t)return!1;for(const o in n.horizontal_walls){const c=n.horizontal_walls[o];if(c[0]==e&&c[1]==t)return!1}for(const o in n.vertical_walls){const c=n.vertical_walls[o];if(c[1]==t){if(c[0]==e)return!1;if(c[0]==e+1)return!1;if(c[0]==e-1)return!1}}return!0}(e,t,n.game)&&(fe(e,t,"wall_right"),fe(e+1,t,"wall_right"),fe(e,t+1,"wall_left"),fe(e+1,t+1,"wall_left"))}(e.row,e.column,e.session)}),t.$$click=()=>function(e,t,n,o){const c=g(),r=localStorage.getItem(K);if(!c||n.current!=c||!r)return;const s="/move/<owner>".replace("<owner>",r);fetch(s,{method:"post",body:JSON.stringify(o(e,t,c))}).then(e=>e.json()).then(console.log).catch(alert)}(e.row,e.column,e.session,ge[he()]),Object(o.c)(t,()=>{return t=e.row,n=e.column,(o=e.session.game).up_player[0]==t&&o.up_player[1]==n?"U":o.down_player[0]==t&&o.down_player[1]==n?"D":void 0;var t,n,o}),Object(c.f)(n=>{const c=function(e,t,n){const o=[te(e,t,n.game)];return"move"==he()&&o.push(function(e,t,n){const o=n.up_player==n.current?n.game.up_player:n.game.down_player;return o[0]==e&&o[1]==t?"player_move_blocked":o&&function(e,t,n){return e==n[0]&&(t==n[1]-1||t==n[1]+1)||t==n[1]&&(e==n[0]-1||e==n[0]+1)}(e,t,o)?function(e,t,n,o){if(e==n[0]){const c=t>n[1]?n[1]:t;for(let t in o.vertical_walls){const n=o.vertical_walls[t];if(c==n[1]&&(e==n[0]||e-1==n[0]))return!0}}if(t==n[1]){const c=e>n[0]?n[0]:e;for(let e in o.horizontal_walls){const n=o.horizontal_walls[e];if(c==n[0]&&(t==n[1]||t-1==n[1]))return!0}}return!1}(e,t,o,n.game)?"player_move_blocked":"player_move":""}(e,t,n)),"tile"+o.join(" ")}(e.row,e.column,e.session),r="".concat(e.row).concat(e.column);return c!==n._v$&&Object(o.a)(t,n._v$=c),r!==n._v$2&&Object(o.f)(t,"id",n._v$2=r),n},{_v$:void 0,_v$2:void 0}),t})()}function Oe(e){return(()=>{const t=ue.cloneNode(!0);return Object(o.c)(t,Object(c.d)(c.a,{each:be,children:t=>Object(c.d)(pe,{column:t,get row(){return e.row},get session(){return e.game}})})),t})()}function je(e){function t(t){return e.session.current==t?"color:red":""}return(()=>{const n=de.cloneNode(!0),r=n.firstChild.firstChild,s=r.firstChild,i=s.nextSibling,l=(i.nextSibling,r.nextSibling),a=l.nextSibling,u=a.firstChild,d=u.nextSibling,b=(d.nextSibling,a.nextSibling.firstChild),g=b.nextSibling,h=g.nextSibling;return Object(o.c)(r,()=>e.session.up_player,s),Object(o.c)(r,()=>e.session.game.up_player_free_walls,i),Object(o.c)(l,Object(c.d)(c.a,{each:be,children:t=>Object(c.d)(Oe,{row:t,get game(){return e.session}})})),Object(o.c)(a,()=>e.session.down_player,u),Object(o.c)(a,()=>e.session.game.down_player_free_walls,d),b.$$click=()=>ve("move"),g.$$click=()=>ve("hW"),h.$$click=()=>ve("vW"),Object(c.f)(c=>{const s=z()?"display:none;":"",i=t(e.session.up_player),l=t(e.session.down_player),u="move"===he()?"color:red;":"",d="hW"===he()?"color:red;":"",v="vW"===he()?"color:red;":"";return c._v$3=Object(o.g)(n,s,c._v$3),c._v$4=Object(o.g)(r,i,c._v$4),c._v$5=Object(o.g)(a,l,c._v$5),c._v$6=Object(o.g)(b,u,c._v$6),c._v$7=Object(o.g)(g,d,c._v$7),c._v$8=Object(o.g)(h,v,c._v$8),c},{_v$3:void 0,_v$4:void 0,_v$5:void 0,_v$6:void 0,_v$7:void 0,_v$8:void 0}),n})()}Object(o.b)(["click"]);const _e=Object(o.h)('<div class="full_screen_centered"><div class="form_container"></div></div>',4),we=Object(o.h)('<div class="covering-panel"><div class="spin"></div></div>',4),ye=Object(o.h)("<h1>Looking for opponent ...</h1>",2),Se=Object(o.h)("<hr>",1),ke=Object(o.h)("<h3>Press Esc to cancel</h3>",2),xe=Object(o.h)('<div class="full_screen_centered"></div>',2);function $e(){const[e,t]=Object(c.h)(!0),[n,r]=Object(c.h)(!1);return[Object(c.d)(p,{left:{text:"Sign In",click:()=>{t(!e()),r(!1)}},right:{text:"Create New Account",click:()=>{r(!n()),t(!1)}}}),(()=>{const t=_e.cloneNode(!0),r=t.firstChild;return Object(o.c)(r,Object(c.d)(k,{get show(){return e()}}),null),Object(o.c)(r,Object(c.d)(x,{get show(){return n()}}),null),t})()]}const[Ne,Ie]=Object(c.h)(null),[Ce,Ee]=Object(c.h)(!1);function Le(){const e=()=>{const e=e=>{console.log(e.key),"Escape"===e.key&&function(){const e={owner:g(),game:"Unknown"};fetch("/create_lobby",{method:"post",body:JSON.stringify(e)}).then(e=>{localStorage.removeItem("activeLobby"),Ee(!1)}).catch(e=>{localStorage.removeItem("activeLobby"),Ee(!1),console.log(e)})}()};return Object(c.k)(()=>{document.addEventListener("keydown",e)}),Object(c.j)(()=>{document.removeEventListener("keydown",e)}),[we.cloneNode(!0),Object(c.d)(p,{}),ye.cloneNode(!0),Se.cloneNode(!0),ke.cloneNode(!0)]};return Object(c.d)(c.c,{get children(){return[Object(c.d)(c.b,{get when(){return Ce()},get children(){return Object(c.d)(e,{})}}),Object(c.d)(c.b,{get when(){return!Ce()},get children(){return[Object(c.d)(p,{left:{text:"Game VS CPU",click:()=>{var e;e=Ie,fetch("/join/<owner>".replace("<owner>","|QCPU|")).then(e=>e.json()).then(t=>{t&&X(t,e)}).catch(console.log)}},right:{text:"Create Lobby",click:()=>{!function(e){const t={owner:g(),game:"Quoridor"};fetch("/create_lobby",{method:"post",body:JSON.stringify(t)}).then(e=>e.json()).then(t=>{t&&(localStorage.setItem("activeLobby",t),Ee(!0),oe(t,e))}).catch(console.log)}(Ie)}}}),(()=>{const e=xe.cloneNode(!0);return Object(o.c)(e,Object(c.d)(le,{gameSetter:Ie})),e})()]}})]}})}function Je(){return[Object(c.d)(p,{right:{text:"Concede",style:"color:red;",click:()=>{!function(){const e=g(),t=localStorage.getItem(K);if(!e||!t)return;const n="/move/<owner>".replace("<owner>",t);fetch(n,{method:"post",body:JSON.stringify({Concede:e})})}()}},get left(){return Object(o.d)(()=>{var e,t;return"|QCPU|"==(null===(e=Ne())||void 0===e||null===(t=e.ActiveQuoridor)||void 0===t?void 0:t.down_player)},!0)()?void 0:{text:z()?"Back to Game":"Open Chat ".concat(Q()?Q():""),style:Q()?"color: red;":"",click:()=>{A(!z()),z()&&W(0)}}}}),Object(c.d)((function(e){if(e.session)return Object(c.k)(()=>{return e=Ie,Y(),void sessionStorage.setItem("stateBackupTimeout",window.setInterval(()=>{ee(e)},15e3).toString());var e}),Object(c.j)(Y),e.session.ActiveQuoridor?Object(c.d)(je,{get session(){return e.session.ActiveQuoridor}}):[]}),{get session(){return Ne()}}),(()=>{const e=xe.cloneNode(!0);return Object(o.c)(e,(()=>{const e=Object(o.d)(()=>{var e,t;return"|QCPU|"==(null===(e=Ne())||void 0===e||null===(t=e.ActiveQuoridor)||void 0===t?void 0:t.down_player)},!0);return()=>e()?[]:Object(c.d)(R,{get new_msg(){return U()}})})()),e})()]}var Qe=function(){return function(e,t){const n=localStorage.getItem(K);if(n&&(M(),ee(t),null!==e()))new N("/game_events/<owner>".replace("<owner>",n),(e,n)=>{Z(e,n,t)})}(Ne,Ie),function(e){const t=localStorage.getItem("activeLobby");t&&(Ee(!0),oe(t,e))}(Ie),Object(c.d)(c.c,{get children(){return[Object(c.d)(c.b,{get when(){return!l()},get children(){return Object(c.d)($e,{})}}),Object(c.d)(c.b,{get when(){return Object(o.d)(()=>!!l(),!0)()&&!Ne()},get children(){return Object(c.d)(Le,{})}}),Object(c.d)(c.b,{get when(){return Object(o.d)(()=>!!l(),!0)()&&Ne()},get children(){return Object(c.d)(Je,{})}})]}})};Boolean("localhost"===window.location.hostname||"[::1]"===window.location.hostname||window.location.hostname.match(/^127(?:\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}$/));Object(o.e)(Qe,document.getElementById("root")),"serviceWorker"in navigator&&navigator.serviceWorker.ready.then(e=>{e.unregister()})}],[[3,1,2]]]);