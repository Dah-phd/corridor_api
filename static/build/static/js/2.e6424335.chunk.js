(window.webpackJsonp=window.webpackJsonp||[]).push([[2],[function(e,t,n){"use strict";n.d(t,"a",(function(){return a})),n.d(t,"b",(function(){return l})),n.d(t,"c",(function(){return d})),n.d(t,"d",(function(){return o})),n.d(t,"e",(function(){return i})),n.d(t,"f",(function(){return c})),n.d(t,"g",(function(){return f})),n.d(t,"h",(function(){return u}));var r=n(1);new Set(["className","value","readOnly","formNoValidate","isMap","noModule","playsInline","allowfullscreen","async","autofocus","autoplay","checked","controls","default","disabled","formnovalidate","hidden","indeterminate","ismap","loop","multiple","muted","nomodule","novalidate","open","playsinline","readonly","required","reversed","seamless","selected"]),new Set(["innerHTML","textContent","innerText","children"]),new Set(["beforeinput","click","dblclick","contextmenu","focusin","focusout","input","keydown","keyup","mousedown","mousemove","mouseout","mouseover","mouseup","pointerdown","pointermove","pointerout","pointerover","pointerup","touchend","touchmove","touchstart"]),new Set(["altGlyph","altGlyphDef","altGlyphItem","animate","animateColor","animateMotion","animateTransform","circle","clipPath","color-profile","cursor","defs","desc","ellipse","feBlend","feColorMatrix","feComponentTransfer","feComposite","feConvolveMatrix","feDiffuseLighting","feDisplacementMap","feDistantLight","feFlood","feFuncA","feFuncB","feFuncG","feFuncR","feGaussianBlur","feImage","feMerge","feMergeNode","feMorphology","feOffset","fePointLight","feSpecularLighting","feSpotLight","feTile","feTurbulence","filter","font","font-face","font-face-format","font-face-name","font-face-src","font-face-uri","foreignObject","g","glyph","glyphRef","hkern","image","line","linearGradient","marker","mask","metadata","missing-glyph","mpath","path","pattern","polygon","polyline","radialGradient","rect","set","stop","svg","switch","symbol","text","textPath","tref","tspan","use","view","vkern"]),new Set(["html","base","head","link","meta","style","title","body","address","article","aside","footer","header","main","nav","section","body","blockquote","dd","div","dl","dt","figcaption","figure","hr","li","ol","p","pre","ul","a","abbr","b","bdi","bdo","br","cite","code","data","dfn","em","i","kbd","mark","q","rp","rt","ruby","s","samp","small","span","strong","sub","sup","time","u","var","wbr","area","audio","img","map","track","video","embed","iframe","object","param","picture","portal","source","svg","math","canvas","noscript","script","del","ins","caption","col","colgroup","table","tbody","td","tfoot","th","thead","tr","button","datalist","fieldset","form","input","label","legend","meter","optgroup","option","output","progress","select","textarea","details","dialog","menu","summary","details","slot","template","acronym","applet","basefont","bgsound","big","blink","center","content","dir","font","frame","frameset","hgroup","image","keygen","marquee","menuitem","nobr","noembed","noframes","plaintext","rb","rtc","shadow","spacer","strike","tt","xmp","a","abbr","acronym","address","applet","area","article","aside","audio","b","base","basefont","bdi","bdo","bgsound","big","blink","blockquote","body","br","button","canvas","caption","center","cite","code","col","colgroup","content","data","datalist","dd","del","details","dfn","dialog","dir","div","dl","dt","em","embed","fieldset","figcaption","figure","font","footer","form","frame","frameset","head","header","hgroup","hr","html","i","iframe","image","img","input","ins","kbd","keygen","label","legend","li","link","main","map","mark","marquee","menu","menuitem","meta","meter","nav","nobr","noembed","noframes","noscript","object","ol","optgroup","option","output","p","param","picture","plaintext","portal","pre","progress","q","rb","rp","rt","rtc","ruby","s","samp","script","section","select","shadow","slot","small","source","spacer","span","strike","strong","style","sub","summary","sup","table","tbody","td","template","textarea","tfoot","th","thead","time","title","tr","track","tt","u","ul","var","video","wbr","xmp","input"]);function o(e,t){return Object(r.e)(e,void 0,t?void 0:{equals:t})}const s="_$DX_DELEGATE";function i(e,t,n){let o;return Object(r.g)(r=>{o=r,t===document?e():d(t,e(),t.firstChild?null:void 0,n)}),()=>{o(),t.textContent=""}}function u(e,t,n){const r=document.createElement("template");r.innerHTML=e;let o=r.content.firstChild;return n&&(o=o.firstChild),o}function l(e){let t=arguments.length>1&&void 0!==arguments[1]?arguments[1]:window.document;const n=t[s]||(t[s]=new Set);for(let r=0,o=e.length;r<o;r++){const o=e[r];n.has(o)||(n.add(o),t.addEventListener(o,p))}}function c(e,t,n){null==n?e.removeAttribute(t):e.setAttribute(t,n)}function a(e,t){null==t?e.removeAttribute("class"):e.className=t}function f(e,t){let n=arguments.length>2&&void 0!==arguments[2]?arguments[2]:{};const r=e.style,o="string"===typeof n;if(null==t&&o||"string"===typeof t)return r.cssText=t;let s,i;for(i in o&&(r.cssText=void 0,n={}),t||(t={}),n)null==t[i]&&r.removeProperty(i),delete n[i];for(i in t)s=t[i],s!==n[i]&&(r.setProperty(i,s),n[i]=s);return n}function d(e,t,n,o){if(void 0===n||o||(o=[]),"function"!==typeof t)return h(e,t,o,n);Object(r.f)(r=>h(e,t(),r,n),o)}function p(e){const t="$$".concat(e.type);let n=e.composedPath&&e.composedPath()[0]||e.target;for(e.target!==n&&Object.defineProperty(e,"target",{configurable:!0,value:n}),Object.defineProperty(e,"currentTarget",{configurable:!0,get:()=>n||document}),r.m.registry&&!r.m.done&&(r.m.done=!0,document.querySelectorAll("[id^=pl-]").forEach(e=>e.remove()));null!==n;){const r=n[t];if(r&&!n.disabled){const o=n["".concat(t,"Data")];if(void 0!==o?r(o,e):r(e),e.cancelBubble)return}n=n.host&&n.host!==n&&n.host instanceof Node?n.host:n.parentNode}}function h(e,t,n,o,s){for(r.m.context&&!n&&(n=[...e.childNodes]);"function"===typeof n;)n=n();if(t===n)return n;const i=typeof t,u=void 0!==o;if(e=u&&n[0]&&n[0].parentNode||e,"string"===i||"number"===i){if(r.m.context)return n;if("number"===i&&(t=t.toString()),u){let r=n[0];r&&3===r.nodeType?r.data=t:r=document.createTextNode(t),n=b(e,n,o,r)}else n=""!==n&&"string"===typeof n?e.firstChild.data=t:e.textContent=t}else if(null==t||"boolean"===i){if(r.m.context)return n;n=b(e,n,o)}else{if("function"===i)return Object(r.f)(()=>{let r=t();for(;"function"===typeof r;)r=r();n=h(e,r,n,o)}),()=>n;if(Array.isArray(t)){const i=[];if(function e(t,n,r){let o=!1;for(let s=0,i=n.length;s<i;s++){let i,u=n[s];if(u instanceof Node)t.push(u);else if(null==u||!0===u||!1===u);else if(Array.isArray(u))o=e(t,u)||o;else if("string"===(i=typeof u))t.push(document.createTextNode(u));else if("function"===i)if(r){for(;"function"===typeof u;)u=u();o=e(t,Array.isArray(u)?u:[u])||o}else t.push(u),o=!0;else t.push(document.createTextNode(u.toString()))}return o}(i,t,s))return Object(r.f)(()=>n=h(e,i,n,o,!0)),()=>n;if(r.m.context)for(let e=0;e<i.length;e++)if(i[e].parentNode)return n=i;if(0===i.length){if(n=b(e,n,o),u)return n}else Array.isArray(n)?0===n.length?g(e,i,o):function(e,t,n){let r=n.length,o=t.length,s=r,i=0,u=0,l=t[o-1].nextSibling,c=null;for(;i<o||u<s;)if(t[i]!==n[u]){for(;t[o-1]===n[s-1];)o--,s--;if(o===i){const t=s<r?u?n[u-1].nextSibling:n[s-u]:l;for(;u<s;)e.insertBefore(n[u++],t)}else if(s===u)for(;i<o;)c&&c.has(t[i])||t[i].remove(),i++;else if(t[i]===n[s-1]&&n[u]===t[o-1]){const r=t[--o].nextSibling;e.insertBefore(n[u++],t[i++].nextSibling),e.insertBefore(n[--s],r),t[o]=n[s]}else{if(!c){c=new Map;let e=u;for(;e<s;)c.set(n[e],e++)}const r=c.get(t[i]);if(null!=r)if(u<r&&r<s){let l,a=i,f=1;for(;++a<o&&a<s&&null!=(l=c.get(t[a]))&&l===r+f;)f++;if(f>r-u){const o=t[i];for(;u<r;)e.insertBefore(n[u++],o)}else e.replaceChild(n[u++],t[i++])}else i++;else t[i++].remove()}}else i++,u++}(e,n,i):(n&&b(e),g(e,i));n=i}else if(t instanceof Node){if(r.m.context&&t.parentNode)return n=u?[t]:t;if(Array.isArray(n)){if(u)return n=b(e,n,o,t);b(e,n,null,t)}else null!=n&&""!==n&&e.firstChild?e.replaceChild(t,e.firstChild):e.appendChild(t);n=t}}return n}function g(e,t,n){for(let r=0,o=t.length;r<o;r++)e.insertBefore(t[r],n)}function b(e,t,n,r){if(void 0===n)return e.textContent="";const o=r||document.createTextNode("");if(t.length){let r=!1;for(let s=t.length-1;s>=0;s--){const i=t[s];if(o!==i){const t=i.parentNode===e;r||s?t&&i.remove():t?e.replaceChild(o,i):e.insertBefore(o,n)}else r=!0}}else e.insertBefore(o,n);return[o]}},function(e,t,n){"use strict";function r(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function o(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?r(Object(n),!0).forEach((function(t){s(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):r(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function s(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}n.d(t,"a",(function(){return fe})),n.d(t,"b",(function(){return pe})),n.d(t,"c",(function(){return de})),n.d(t,"d",(function(){return se})),n.d(t,"e",(function(){return M})),n.d(t,"f",(function(){return T})),n.d(t,"g",(function(){return C})),n.d(t,"h",(function(){return q})),n.d(t,"i",(function(){return oe})),n.d(t,"j",(function(){return ce})),n.d(t,"k",(function(){return V})),n.d(t,"l",(function(){return B})),n.d(t,"m",(function(){return i})),n.d(t,"n",(function(){return ae})),n.d(t,"o",(function(){return E}));const i={};function u(e){i.context=e}const l=(e,t)=>e===t,c=Symbol("solid-proxy"),a=Symbol("solid-track"),f=(Symbol("solid-dev-component"),{equals:l});let d=null,p=K;const h={},g={owned:null,cleanups:null,context:null,owner:null},[b,m]=q(!1);var y=null;let v,w=null,S=null,x=null,k=null,O=null,A=null,j=null,P=0;function C(e,t){const n=k,r=y,o=0===e.length?g:{owned:null,cleanups:null,context:null,owner:t||r};y=o,k=null;try{return _(()=>e(()=>W(o)),!0)}finally{k=n,y=r}}function q(e,t){t=t?Object.assign({},f,t):f;const n={value:e,observers:null,observerSlots:null,pending:h,comparator:t.equals||void 0};return[z.bind(n),e=>("function"===typeof e&&(e=w&&w.running&&w.sources.has(n)?e(n.pending!==h?n.pending:n.tValue):e(n.pending!==h?n.pending:n.value)),I(n,e))]}function N(e,t,n){const r=J(e,t,!0,1);S&&w&&w.running?A.push(r):$(r)}function T(e,t,n){const r=J(e,t,!1,1);S&&w&&w.running?A.push(r):$(r)}function M(e,t,n){n=n?Object.assign({},f,n):f;const r=J(e,t,!0,0);return r.pending=h,r.observers=null,r.observerSlots=null,r.comparator=n.equals||void 0,S&&w&&w.running?(r.tState=1,A.push(r)):$(r),z.bind(r)}function D(e){if(O)return e();let t;const n=O=[];try{t=e()}finally{O=null}return _(()=>{for(let e=0;e<n.length;e+=1){const t=n[e];if(t.pending!==h){const e=t.pending;t.pending=h,I(t,e)}}},!1),t}function E(e){let t,n=k;return k=null,t=e(),k=n,t}function B(e){!function(e,t,n){p=X;const r=J(e,t,!1,1),o=v&&Z(y,v.id);o&&(r.suspense=o),r.user=!0,j?j.push(r):$(r)}(()=>E(e))}function V(e){return null===y||(null===y.cleanups?y.cleanups=[e]:y.cleanups.push(e)),e}function L(e){if(w&&w.running)return e(),w.done;const t=k,n=y;return Promise.resolve().then(()=>{let r;return k=t,y=n,(S||v)&&(r=w||(w={sources:new Set,effects:[],promises:new Set,disposed:new Set,queue:new Set,running:!0}),r.done||(r.done=new Promise(e=>r.resolve=e)),r.running=!0),D(e),k=y=null,r?r.done:void 0})}function G(e){const t=Symbol("context");return{id:t,Provider:ee(t),defaultValue:e}}function F(e){const t=M(e);return M(()=>function e(t){if("function"===typeof t&&!t.length)return e(t());if(Array.isArray(t)){const n=[];for(let r=0;r<t.length;r++){const o=e(t[r]);Array.isArray(o)?n.push.apply(n,o):n.push(o)}return n}return t}(t()))}function z(){const e=w&&w.running;if(this.sources&&(!e&&this.state||e&&this.tState)){const t=A;A=null,!e&&1===this.state||e&&1===this.tState?$(this):Q(this),A=t}if(k){const e=this.observers?this.observers.length:0;k.sources?(k.sources.push(this),k.sourceSlots.push(e)):(k.sources=[this],k.sourceSlots=[e]),this.observers?(this.observers.push(k),this.observerSlots.push(k.sources.length-1)):(this.observers=[k],this.observerSlots=[k.sources.length-1])}return e&&w.sources.has(this)?this.tValue:this.value}function I(e,t,n){if(O)return e.pending===h&&O.push(e),e.pending=t,t;if(e.comparator)if(w&&w.running&&w.sources.has(e)){if(e.comparator(e.tValue,t))return t}else if(e.comparator(e.value,t))return t;let r=!1;return w?(r=w.running,(r||!n&&w.sources.has(e))&&(w.sources.add(e),e.tValue=t),r||(e.value=t)):e.value=t,e.observers&&e.observers.length&&_(()=>{for(let t=0;t<e.observers.length;t+=1){const n=e.observers[t];r&&w.disposed.has(n)||((r&&!n.tState||!r&&!n.state)&&(n.pure?A.push(n):j.push(n),n.observers&&U(n)),r?n.tState=1:n.state=1)}if(A.length>1e6)throw A=[],new Error},!1),t}function $(e){if(!e.fn)return;W(e);const t=y,n=k,r=P;k=y=e,H(e,w&&w.running&&w.sources.has(e)?e.tValue:e.value,r),w&&!w.running&&w.sources.has(e)&&queueMicrotask(()=>{_(()=>{w&&(w.running=!0),H(e,e.tValue,r)},!1)}),k=n,y=t}function H(e,t,n){let r;try{r=e.fn(t)}catch(o){Y(o)}(!e.updatedAt||e.updatedAt<=n)&&(e.observers&&e.observers.length?I(e,r,!0):w&&w.running&&e.pure?(w.sources.add(e),e.tValue=r):e.value=r,e.updatedAt=n)}function J(e,t,n){let r=arguments.length>3&&void 0!==arguments[3]?arguments[3]:1;const o={fn:e,state:r,updatedAt:null,owned:null,sources:null,sourceSlots:null,cleanups:null,value:t,owner:y,context:null,pure:n};if(w&&w.running&&(o.state=0,o.tState=r),null===y||y!==g&&(w&&w.running&&y.pure?y.tOwned?y.tOwned.push(o):y.tOwned=[o]:y.owned?y.owned.push(o):y.owned=[o]),x){const[e,t]=q(void 0,{equals:!1}),n=x(o.fn,t);V(()=>n.dispose());const r=()=>L(t).then(()=>s.dispose()),s=x(o.fn,r);o.fn=t=>(e(),w&&w.running?s.track(t):n.track(t))}return o}function R(e){const t=w&&w.running;if(!t&&0===e.state||t&&0===e.tState)return;if(!t&&2===e.state||t&&2===e.tState)return Q(e);if(e.suspense&&E(e.suspense.inFallback))return e.suspense.effects.push(e);const n=[e];for(;(e=e.owner)&&(!e.updatedAt||e.updatedAt<P);){if(t&&w.disposed.has(e))return;(!t&&e.state||t&&e.tState)&&n.push(e)}for(let r=n.length-1;r>=0;r--){if(e=n[r],t){let t=e,o=n[r+1];for(;(t=t.owner)&&t!==o;)if(w.disposed.has(t))return}if(!t&&1===e.state||t&&1===e.tState)$(e);else if(!t&&2===e.state||t&&2===e.tState){const t=A;A=null,Q(e,n[0]),A=t}}}function _(e,t){if(A)return e();let n=!1;t||(A=[]),j?n=!0:j=[],P++;try{const t=e();return function(e){A&&(S&&w&&w.running?function(e){for(let t=0;t<e.length;t++){const n=e[t],r=w.queue;r.has(n)||(r.add(n),S(()=>{r.delete(n),_(()=>{w.running=!0,R(n),r.size||(j.push.apply(j,w.effects),w.effects=[])},!1),w&&(w.running=!1)}))}}(A):K(A),A=null);if(e)return;let t;if(w&&w.running){if(w.promises.size||w.queue.size)return w.running=!1,w.effects.push.apply(w.effects,j),j=null,void m(!0);const e=w.sources;t=w.resolve,j.forEach(e=>{"tState"in e&&(e.state=e.tState),delete e.tState}),w=null,D(()=>{e.forEach(e=>{if(e.value=e.tValue,e.owned)for(let t=0,n=e.owned.length;t<n;t++)W(e.owned[t]);e.tOwned&&(e.owned=e.tOwned),delete e.tValue,delete e.tOwned,e.tState=0}),m(!1)})}j.length?D(()=>{p(j),j=null}):j=null;t&&t()}(n),t}catch(r){Y(r)}finally{A=null,n||(j=null)}}function K(e){for(let t=0;t<e.length;t++)R(e[t])}function X(e){let t,n=0;for(t=0;t<e.length;t++){const r=e[t];r.user?e[n++]=r:R(r)}i.context&&u();const r=e.length;for(t=0;t<n;t++)R(e[t]);for(t=r;t<e.length;t++)R(e[t])}function Q(e,t){const n=w&&w.running;n?e.tState=0:e.state=0;for(let r=0;r<e.sources.length;r+=1){const o=e.sources[r];o.sources&&(!n&&1===o.state||n&&1===o.tState?o!==t&&R(o):(!n&&2===o.state||n&&2===o.tState)&&Q(o,t))}}function U(e){const t=w&&w.running;for(let n=0;n<e.observers.length;n+=1){const r=e.observers[n];(!t&&!r.state||t&&!r.tState)&&(t?r.tState=2:r.state=2,r.pure?A.push(r):j.push(r),r.observers&&U(r))}}function W(e){let t;if(e.sources)for(;e.sources.length;){const t=e.sources.pop(),n=e.sourceSlots.pop(),r=t.observers;if(r&&r.length){const e=r.pop(),o=t.observerSlots.pop();n<r.length&&(e.sourceSlots[o]=n,r[n]=e,t.observerSlots[n]=o)}}if(w&&w.running&&e.pure){if(e.tOwned){for(t=0;t<e.tOwned.length;t++)W(e.tOwned[t]);delete e.tOwned}!function e(t,n){n||(t.tState=0,w.disposed.add(t));if(t.owned)for(let r=0;r<t.owned.length;r++)e(t.owned[r])}(e,!0)}else if(e.owned){for(t=0;t<e.owned.length;t++)W(e.owned[t]);e.owned=null}if(e.cleanups){for(t=0;t<e.cleanups.length;t++)e.cleanups[t]();e.cleanups=null}w&&w.running?e.tState=0:e.state=0,e.context=null}function Y(e){const t=d&&Z(y,d);if(!t)throw e;t.forEach(t=>t(e))}function Z(e,t){return e?e.context&&void 0!==e.context[t]?e.context[t]:Z(e.owner,t):void 0}function ee(e){return function(t){let n;return N(()=>n=E(()=>(y.context={[e]:t.value},F(()=>t.children)))),n}}const te=Symbol("fallback");function ne(e){for(let t=0;t<e.length;t++)e[t]()}let re=!1;function oe(){re=!0}function se(e,t){if(re&&i.context){const n=i.context;u(o(o({},i.context),{},{id:"".concat(i.context.id).concat(i.context.count++,"-"),count:0}));const r=E(()=>e(t||{}));return u(n),r}return E(()=>e(t||{}))}function ie(){return!0}const ue={get:(e,t,n)=>t===c?n:e.get(t),has:(e,t)=>e.has(t),set:ie,deleteProperty:ie,getOwnPropertyDescriptor:(e,t)=>({configurable:!0,enumerable:!0,get:()=>e.get(t),set:ie,deleteProperty:ie}),ownKeys:e=>e.keys()};function le(e){return null==(e="function"===typeof e?e():e)?{}:e}function ce(){for(var e=arguments.length,t=new Array(e),n=0;n<e;n++)t[n]=arguments[n];return new Proxy({get(e){for(let n=t.length-1;n>=0;n--){const r=le(t[n])[e];if(void 0!==r)return r}},has(e){for(let n=t.length-1;n>=0;n--)if(e in le(t[n]))return!0;return!1},keys(){const e=[];for(let n=0;n<t.length;n++)e.push(...Object.keys(le(t[n])));return[...new Set(e)]}},ue)}function ae(e){for(var t=arguments.length,n=new Array(t>1?t-1:0),r=1;r<t;r++)n[r-1]=arguments[r];const o=new Set(n.flat()),s=Object.getOwnPropertyDescriptors(e),i=n.map(t=>{const n={};for(let r=0;r<t.length;r++){const o=t[r];Object.defineProperty(n,o,s[o]?s[o]:{get:()=>e[o],set:()=>!0})}return n});return i.push(new Proxy({get:t=>o.has(t)?void 0:e[t],has:t=>!o.has(t)&&t in e,keys:()=>Object.keys(e).filter(e=>!o.has(e))},ue)),i}function fe(e){const t="fallback"in e&&{fallback:()=>e.fallback};return M(function(e,t){let n=arguments.length>2&&void 0!==arguments[2]?arguments[2]:{},r=[],o=[],s=[],i=0,u=t.length>1?[]:null;return V(()=>ne(s)),()=>{let l,c,f=e()||[];return f[a],E(()=>{let e,t,a,p,h,g,b,m,y,v=f.length;if(0===v)0!==i&&(ne(s),s=[],r=[],o=[],i=0,u&&(u=[])),n.fallback&&(r=[te],o[0]=C(e=>(s[0]=e,n.fallback())),i=1);else if(0===i){for(o=new Array(v),c=0;c<v;c++)r[c]=f[c],o[c]=C(d);i=v}else{for(a=new Array(v),p=new Array(v),u&&(h=new Array(v)),g=0,b=Math.min(i,v);g<b&&r[g]===f[g];g++);for(b=i-1,m=v-1;b>=g&&m>=g&&r[b]===f[m];b--,m--)a[m]=o[b],p[m]=s[b],u&&(h[m]=u[b]);for(e=new Map,t=new Array(m+1),c=m;c>=g;c--)y=f[c],l=e.get(y),t[c]=void 0===l?-1:l,e.set(y,c);for(l=g;l<=b;l++)y=r[l],c=e.get(y),void 0!==c&&-1!==c?(a[c]=o[l],p[c]=s[l],u&&(h[c]=u[l]),c=t[c],e.set(y,c)):s[l]();for(c=g;c<v;c++)c in a?(o[c]=a[c],s[c]=p[c],u&&(u[c]=h[c],u[c](c))):o[c]=C(d);o=o.slice(0,i=v),r=f.slice(0)}return o});function d(e){if(s[c]=e,u){const[e,n]=q(c);return u[c]=n,t(f[c],e)}return t(f[c])}}}(()=>e.each,e.children,t||void 0))}function de(e){let t=!1;const n=F(()=>e.children),r=M(()=>{let e=n();Array.isArray(e)||(e=[e]);for(let t=0;t<e.length;t++){const n=e[t].when;if(n)return[t,n,e[t]]}return[-1]},void 0,{equals:(e,n)=>e[0]===n[0]&&(t?e[1]===n[1]:!e[1]===!n[1])&&e[2]===n[2]});return M(()=>{const[n,o,s]=r();if(n<0)return e.fallback;const i=s.children;return(t="function"===typeof i&&i.length>0)?E(()=>i(o)):i})}function pe(e){return e}G()}]]);