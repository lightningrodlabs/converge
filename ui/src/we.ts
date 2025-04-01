import { asyncDerived, pipe, sliceAndJoin, toPromise } from '@holochain-open-dev/stores';
import { LazyHoloHashMap } from '@holochain-open-dev/utils';
import type { AppletHash, AppletServices, AssetInfo, WAL, RecordInfo, WeaveServices } from '@theweave/api';
import type { RoleName, ZomeName, AppClient } from '@holochain/client';
import { getMyDna, hrlWithContextToB64 } from './util';
import type { Deliberation, Proposal } from './converge/converge/types';
import { decode } from '@msgpack/msgpack';

const appPort = import.meta.env.VITE_APP_PORT ? import.meta.env.VITE_APP_PORT : 8888
const adminPort = import.meta.env.VITE_ADMIN_PORT
const url = `ws://localhost:${appPort}`;

const ROLE_NAME = "converge"
const ZOME_NAME = "converge"
const appId = import.meta.env.VITE_APP_ID ? import.meta.env.VITE_APP_ID : 'converge'

const ICON = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" style="enable-background:new 0 0 64 64" xml:space="preserve"><path d="M6 12c0-3.3 2.7-6 6-6h40c3.3 0 6 2.7 6 6v40c0 3.3-2.7 6-6 6H12c-3.3 0-6-2.7-6-6V12z" style="fill:%23fff"/><path d="M4 12c0-4.4 3.6-8 8-8h8v16H8v12h12v12H8v2c0 2.8 5.1 5.1 12 5.8V44h12v7.4c4.4-.7 8.5-2 12-3.8V44h5.6c4-3.3 6.4-7.5 6.4-12h2v20c0 3.3-2.7 6-6 6h-8v2H12c-4.4 0-8-3.6-8-8V12zm28 20v12h12V32H32zm12 0h12V20H44v12zm0-12V8H32v12h12zm-12 0H20v12h12V20z" style="fill:%23acbdc5"/><path d="M32 56H20v-4.2c1.3.1 2.6.2 4 .2 2.8 0 5.4-.2 8-.6V56zm12-8.4V56h12V44h-6.4c-1.6 1.3-3.5 2.6-5.6 3.6z" style="fill:%23597380"/><path d="M20 4h32c4.4 0 8 3.6 8 8v40c0 4.4-3.6 8-8 8h-8v-4h8c2.2 0 4-1.8 4-4V12c0-2.2-1.8-4-4-4H20V4z" style="fill-rule:evenodd;clip-rule:evenodd;fill:%23314a52"/></svg>'
const ICON2 = `<svg width="20" height="20" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 900.000000 876.000000" 
preserveAspectRatio="xMidYMid meet"
style="enable-background:new 0 0 64 64" xml:space="preserve">
<g transform="translate(0.000000,876.000000) scale(0.100000,-0.100000)">
<path d="M4260 8604 c-149 -11 -620 -98 -745 -139 -22 -7 -67 -21 -100 -30
-252 -74 -455 -155 -695 -278 -184 -94 -176 -90 -353 -197 -60 -36 -116 -70
-125 -75 -83 -49 -321 -226 -425 -315 -195 -170 -494 -495 -626 -683 -33 -48
-88 -125 -121 -172 -33 -47 -60 -87 -60 -89 0 -2 -34 -61 -75 -130 -207 -351
-385 -797 -455 -1146 -12 -58 -25 -123 -30 -145 -4 -22 -16 -85 -25 -140 -9
-55 -21 -124 -26 -154 -47 -276 -34 -874 26 -1176 7 -38 21 -108 30 -155 51
-260 66 -319 139 -540 87 -261 225 -567 345 -764 23 -36 41 -69 41 -72 0 -2 8
-17 19 -32 10 -15 51 -76 91 -137 261 -391 609 -754 970 -1011 411 -292 898
-532 1335 -659 263 -76 341 -93 715 -152 165 -26 831 -26 995 0 61 9 155 24
210 33 123 19 216 40 408 91 145 38 460 147 519 179 14 8 32 14 39 14 7 0 96
41 198 92 574 282 975 589 1365 1043 146 170 169 199 258 330 235 344 403 681
535 1075 8 25 18 61 23 80 4 19 18 71 30 115 25 91 42 166 70 310 54 281 68
428 77 769 6 263 -4 422 -43 666 -46 286 -140 651 -224 870 -12 30 -24 64 -27
75 -4 11 -12 29 -19 40 -6 11 -15 31 -19 45 -25 84 -203 431 -293 570 -210
325 -375 532 -616 774 -185 185 -458 402 -659 524 -43 26 -91 55 -105 64 -15
10 -34 21 -42 25 -8 4 -46 25 -85 48 -93 55 -420 215 -520 255 -312 125 -598
206 -895 254 -52 9 -138 23 -190 32 -74 13 -171 17 -440 19 -190 1 -370 1
-400 -1z m589 -444 c64 -6 150 -16 191 -22 41 -6 102 -14 135 -18 33 -4 80
-13 105 -18 25 -6 79 -18 120 -27 84 -18 172 -40 240 -61 75 -23 164 -52 190
-63 14 -5 69 -26 124 -46 291 -106 662 -314 961 -539 278 -209 418 -343 639
-606 122 -146 263 -355 365 -540 62 -113 171 -338 171 -353 0 -7 4 -17 9 -23
4 -5 13 -22 19 -39 6 -16 20 -55 32 -85 30 -78 84 -244 103 -315 31 -118 36
-141 55 -240 19 -95 36 -197 63 -380 18 -115 23 -559 9 -675 -7 -52 -16 -131
-20 -175 -54 -554 -299 -1206 -624 -1659 -192 -267 -232 -317 -361 -454 -187
-200 -580 -510 -815 -645 -500 -287 -881 -417 -1495 -511 -151 -23 -750 -25
-900 -2 -139 21 -263 43 -340 60 -225 51 -197 43 -420 115 -330 107 -652 267
-965 479 -341 233 -583 458 -815 762 -28 35 -69 88 -92 117 -39 49 -104 145
-181 268 -41 65 -172 322 -207 405 -75 175 -154 398 -168 470 -2 14 -14 59
-25 100 -76 274 -122 644 -122 975 0 233 5 297 46 575 37 254 130 603 209 785
7 17 30 71 50 120 46 112 178 366 225 435 10 14 44 66 75 115 153 239 377 501
626 731 99 91 432 332 604 436 192 116 448 241 630 308 77 28 156 57 175 65
101 39 434 117 639 150 52 8 124 15 160 16 36 0 84 4 106 8 58 13 339 13 474
1z"/>
<path d="M4495 7581 c-183 -37 -320 -172 -371 -367 -62 -240 84 -521 316 -608
116 -43 282 -37 385 14 212 103 335 357 280 575 -30 118 -58 167 -149 262 -86
90 -171 124 -316 129 -58 1 -123 -1 -145 -5z"/>
<path d="M2365 7021 c-220 -56 -360 -200 -404 -419 -33 -159 24 -341 143 -457
126 -124 303 -172 478 -131 138 33 273 150 342 296 40 85 40 85 39 205 0 110
-3 126 -30 190 -52 124 -155 240 -252 281 -57 25 -173 54 -211 53 -19 0 -66
-9 -105 -18z"/>
<path d="M6665 7024 c-182 -39 -309 -147 -381 -325 -23 -56 -26 -79 -26 -179
0 -121 9 -159 61 -258 41 -79 153 -184 236 -221 117 -52 294 -52 410 1 200 90
333 336 295 545 -16 92 -37 148 -79 213 -77 118 -174 185 -321 221 -88 22
-102 22 -195 3z"/>
<path d="M3986 6599 c-78 -31 -153 -87 -199 -148 -91 -119 -96 -148 -97 -483
-1 -154 1 -283 4 -286 6 -6 74 5 251 41 197 40 251 47 454 59 333 19 704 -10
996 -78 74 -17 117 -20 133 -10 9 5 12 76 12 280 0 288 -3 313 -54 413 -28 53
-105 131 -165 166 -57 34 -137 67 -159 67 -7 0 -50 -37 -95 -83 -59 -60 -100
-92 -147 -114 -197 -95 -413 -94 -615 2 -74 35 -103 57 -164 119 -40 42 -81
76 -90 75 -9 -1 -38 -10 -65 -20z"/>
<path d="M1827 5988 c-132 -97 -186 -255 -138 -406 59 -184 191 -296 368 -313
45 -4 99 -13 120 -19 119 -38 242 -127 318 -230 20 -28 44 -50 53 -50 10 1 33
26 52 58 65 105 250 277 385 358 66 40 225 117 298 145 64 26 66 29 39 112
-45 135 -175 298 -281 352 -28 14 -56 25 -62 25 -7 0 -43 -33 -80 -72 -68 -74
-131 -114 -244 -155 -80 -29 -315 -25 -415 7 -125 39 -213 99 -287 197 -34 44
-56 42 -126 -9z"/>
<path d="M7252 5985 c-61 -84 -157 -148 -280 -188 -57 -19 -88 -22 -212 -20
-126 1 -155 4 -217 25 -100 34 -175 82 -244 156 -32 34 -63 62 -68 62 -43 0
-158 -91 -225 -179 -68 -89 -127 -234 -110 -273 3 -7 69 -44 147 -82 183 -88
320 -179 431 -286 86 -82 165 -177 179 -212 11 -30 34 -20 70 30 63 84 160
165 242 201 48 21 134 41 199 47 81 7 140 29 201 75 51 38 111 110 137 164 84
171 19 408 -137 499 -24 14 -52 26 -62 26 -10 -1 -33 -21 -51 -45z"/>
<path d="M4335 5560 c-22 -4 -80 -11 -130 -15 -86 -6 -260 -35 -335 -55 -19
-5 -74 -19 -122 -30 -81 -19 -229 -65 -288 -88 -14 -5 -45 -18 -70 -27 -50
-19 -234 -108 -264 -128 -211 -133 -322 -235 -409 -374 -35 -55 -34 -71 7
-125 46 -60 65 -67 90 -31 27 39 152 161 211 206 120 92 165 120 269 171 97
47 183 83 316 131 19 7 64 21 100 30 36 10 85 23 110 30 41 12 104 25 260 54
198 38 623 54 824 31 162 -19 309 -43 411 -67 55 -13 118 -28 140 -33 47 -11
203 -61 260 -84 22 -10 49 -21 60 -25 11 -4 77 -36 148 -70 161 -79 336 -207
432 -317 38 -43 77 -79 86 -79 18 0 99 99 99 120 0 7 -19 41 -41 76 -74 115
-256 278 -399 358 -86 47 -227 113 -300 139 -281 101 -479 149 -800 193 -103
14 -609 21 -665 9z"/>
<path d="M1650 5181 c0 -11 4 -41 9 -68 6 -26 17 -95 26 -153 15 -100 27 -170
60 -352 8 -46 21 -128 30 -183 9 -55 22 -138 30 -185 7 -47 21 -132 30 -190
26 -166 34 -204 56 -246 41 -75 87 -122 157 -155 l67 -33 315 -3 c341 -4 369
-1 315 39 -108 80 -195 219 -211 337 l-7 51 65 0 65 0 41 -57 c108 -153 219
-242 477 -384 87 -48 210 -160 239 -219 10 -19 27 -64 37 -100 18 -61 19 -106
19 -750 l0 -685 25 -37 c42 -61 130 -73 183 -24 52 47 51 32 52 834 0 410 4
750 9 755 4 4 35 1 69 -7 33 -9 69 -16 81 -16 11 0 25 -9 31 -19 6 -12 10
-121 10 -283 0 -276 6 -321 48 -405 46 -89 138 -131 243 -109 42 9 62 20 99
57 64 64 70 97 70 400 0 154 4 258 10 270 10 18 23 19 206 19 114 0 203 -4
214 -10 19 -10 20 -24 17 -288 -2 -272 -2 -278 21 -324 49 -98 147 -149 242
-126 80 20 128 62 169 148 26 55 26 57 31 355 l5 300 25 7 c142 40 149 41 165
28 13 -11 15 -102 15 -775 l0 -762 33 -37 c30 -35 37 -38 84 -38 54 0 83 14
111 54 15 21 17 87 17 723 0 745 -2 711 46 820 29 67 112 158 190 209 19 13
74 44 121 69 194 102 313 206 420 366 27 40 29 41 88 41 69 0 76 -7 61 -66
-27 -107 -85 -209 -164 -287 -32 -32 -56 -61 -52 -65 4 -4 149 -7 324 -7 l316
0 68 33 c110 55 177 156 193 292 3 25 11 83 19 130 8 47 21 130 30 185 8 55
22 136 30 180 17 95 38 221 60 365 9 58 23 143 30 190 8 47 17 103 20 125 4
22 9 50 12 63 8 34 -20 28 -64 -14 -77 -72 -181 -108 -368 -130 -128 -14 -146
-31 -484 -452 -74 -91 -131 -154 -144 -157 -12 -2 -186 -6 -387 -7 -353 -3
-367 -4 -410 -26 -49 -24 -85 -69 -101 -126 -22 -80 48 -190 138 -215 22 -6
139 -11 276 -11 187 0 237 -3 237 -13 0 -15 -126 -116 -202 -161 -56 -34 -225
-113 -267 -126 -14 -5 -51 -18 -81 -30 -91 -35 -220 -72 -365 -104 -331 -72
-392 -78 -775 -73 -316 4 -423 12 -575 46 -27 7 -81 18 -120 27 -123 27 -263
65 -327 90 -21 8 -55 20 -75 28 -216 81 -347 151 -465 249 -81 67 -80 67 200
67 246 0 249 0 299 26 127 63 149 210 46 305 -50 47 -72 49 -469 49 l-373 0
-37 38 c-20 20 -62 69 -94 107 -378 453 -384 459 -498 470 -167 16 -278 55
-362 125 -53 44 -70 49 -70 21z"/>
<path d="M1270 4921 c-28 -11 -57 -34 -77 -60 -28 -37 -33 -51 -33 -99 0 -31
7 -90 15 -132 7 -41 21 -122 31 -180 9 -58 22 -136 29 -175 8 -38 21 -113 30
-165 40 -241 50 -295 80 -445 37 -177 46 -205 92 -285 39 -67 97 -129 183
-195 36 -27 68 -54 72 -60 4 -5 8 -195 8 -421 l0 -411 30 -41 c63 -88 192 -67
217 36 5 20 10 210 12 422 1 284 5 387 14 392 7 4 183 8 392 8 375 0 381 0
425 -22 88 -45 84 -29 90 -456 3 -258 9 -385 17 -403 52 -118 194 -158 300
-86 65 44 63 24 63 572 0 549 4 515 -68 604 -21 26 -56 53 -86 67 l-51 24
-484 0 c-405 0 -493 3 -543 16 -125 32 -221 96 -283 189 -45 68 -68 135 -90
255 -9 52 -23 127 -30 165 -7 39 -21 111 -30 160 -15 87 -29 160 -59 315 -8
41 -22 120 -31 175 -26 155 -51 205 -114 231 -56 24 -74 24 -121 5z"/>
<path d="M7813 4914 c-65 -28 -85 -82 -128 -339 -9 -55 -22 -131 -30 -170 -7
-38 -21 -117 -30 -175 -8 -58 -22 -136 -30 -175 -63 -315 -63 -313 -87 -365
-41 -86 -129 -182 -203 -220 -22 -11 -71 -29 -110 -40 -65 -19 -102 -20 -540
-21 -418 0 -475 -2 -512 -17 -62 -26 -117 -77 -149 -140 l-29 -57 -3 -477 c-3
-462 -2 -478 17 -517 70 -137 278 -125 352 20 18 36 19 62 19 415 l0 376 38
38 c21 21 55 43 77 49 58 16 767 15 783 -1 9 -9 12 -114 12 -407 1 -352 3
-400 18 -431 21 -43 53 -60 116 -60 44 0 53 4 88 40 l38 41 0 424 0 425 70 51
c149 108 233 241 268 424 11 55 27 145 37 200 9 55 23 129 30 165 8 36 21 108
30 160 9 52 22 124 30 160 7 36 21 110 30 165 9 55 23 133 31 173 25 129 16
198 -35 255 -38 44 -134 59 -198 31z"/>
<path d="M2443 2882 c-9 -6 -13 -104 -13 -404 -1 -348 1 -400 15 -428 34 -66
140 -76 197 -20 l28 28 0 400 c0 220 -3 407 -6 416 -5 13 -24 16 -108 16 -55
0 -106 -4 -113 -8z"/>
<path d="M6567 2883 c-4 -3 -7 -191 -7 -417 l1 -411 22 -28 c18 -21 34 -29 73
-33 59 -6 89 8 121 55 23 34 23 35 23 422 0 213 -3 394 -6 403 -5 13 -25 16
-113 16 -59 0 -111 -3 -114 -7z"/>
</g>
</svg>`
const CARD_ICON_SRC = `data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path d="M0 96C0 60.7 28.7 32 64 32H448c35.3 0 64 28.7 64 64V416c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V96zm64 0v64h64V96H64zm384 0H192v64H448V96zM64 224v64h64V224H64zm384 0H192v64H448V224zM64 352v64h64V352H64zm384 0H192v64H448V352z"/></svg>`
const MINILOGO = '<svg width="200" height="100" xmlns="http://www.w3.org/2000/svg"><rect x="10" y="10" width="180" height="80" fill="darkgreen" stroke="lightgreen" stroke-width="5"/></svg>'

export const appletServices: AppletServices = {
  // Types of attachment that this Applet offers for other Applets to be created
  creatables: {
    'Deliberation': {
      label: "Deliberation",
      icon_src: MINILOGO,
    },
    'Proposal': {
      label: "Proposal",
      icon_src: MINILOGO,
    }
  },
  // Types of UI widgets/blocks that this Applet supports
  blockTypes: {
    'my_deliberations': {
      label: 'My Deliberations',
      icon_src: 
      `<svg xmlns="http://www.w3.org/2000/svg" height="16" width="16" viewBox="0 0 512 512"><!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path d="M0 96C0 60.7 28.7 32 64 32H448c35.3 0 64 28.7 64 64V416c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V96zm64 0v64h64V96H64zm384 0H192v64H448V96zM64 224v64h64V224H64zm384 0H192v64H448V224zM64 352v64h64V352H64zm384 0H192v64H448V352z"/></svg>`,
      view: "applet-view",
    },      
  },
  getAssetInfo: async (
    appletClient: AppClient,
    wal: WAL,
    recordInfo: RecordInfo
  ): Promise<AssetInfo | undefined> => {
    console.log("looking for asset!")
    // return {
    //   icon_src: `data:image/svg+xml;utf8,${ICON2}`,
    //   name: "Deliberation: hello world",
    // };
    const entryType: string = recordInfo.entryType
    if (entryType == "deliberation") {
      let dnaHash = await getMyDna(ROLE_NAME, appletClient)
      let deliberation: Deliberation;
      let record: any;
      
      try {
        record = await appletClient.callZome({
          cap_secret: null,
          role_name: 'converge',
          zome_name: 'converge',
          fn_name: 'get_deliberation',
          payload: wal.hrl[1],
        });
        if (record) {
          console.log(record)
          deliberation = decode((record.record.entry as any).Present.entry) as Deliberation;
        }
      } catch (e) {
        console.log(e)
      }
      
      return {
        icon_src: `data:image/svg+xml;utf8,${ICON2}`,
        name: "Deliberation: " + deliberation.title,
      };
    } else if (entryType == "proposal") {
      let proposal: any;
      let record: any;

      try {
        record = await appletClient.callZome({
          cap_secret: null,
          role_name: 'converge',
          zome_name: 'converge',
          fn_name: 'get_proposal',
          payload: wal.hrl[1],
        });
        if (record) {
          proposal = decode((record.entry as any).Present.entry) as Proposal;
        }
      } catch (e) {
        console.log(e)
      }

      return {
        icon_src: `data:image/svg+xml;utf8,${ICON2}`,
        name: "Proposal: " + proposal.title,
      };
    } else {
      throw new Error("unknown entry type:"+ entryType)
    }
  },
  search: async (
    appletClient: AppClient,
      appletHash: AppletHash,
      weaveServices: WeaveServices,
      searchFilter: string
  ): Promise<Array<WAL>> => {
    let hashes: WAL[];
    let dnaHash = await getMyDna(ROLE_NAME, appletClient)
    
    try {
      const records = await appletClient.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'search_all_deliberations',
        payload: searchFilter,
      });
      hashes = records
      .map((r) => ({ hrl: [dnaHash, r], context: {} }));
    } catch (e) {
      console.log(e)
    }
    return hashes
  },
    // attachmentTypes: async (
    //   appletClient: AppAgentClient,
    //   appletHash: AppletHash,
    //   weServices: WeServices
    // ) => ({
    //   // No way to specify the context so we can't create a board.
    //   deliberation: {
    //     label: "Deliberation",
    //     icon_src: CARD_ICON_SRC,
    //     async create(attachToHrlWithContext: HrlWithContext) {
    //       const hrlB64 = hrlWithContextToB64(attachToHrlWithContext)
    //       const dnaHash = await getMyDna(ROLE_NAME, appletClient)
    //       console.log("context", attachToHrlWithContext)

    //       // const deliberationEntry: Deliberation = { 
    //       //   title: attachToHrlWithContext.context.title!,
    //       //   description: attachToHrlWithContext.context.description!,
    //       //   settings: JSON.stringify(attachToHrlWithContext.context.settings!),
    //       //   attachments: [hrlB64]
    //       // };

    //       const deliberationEntry: Deliberation = { 
    //         title: "Deliberation",
    //         description: "",
    //         settings: "",
    //         attachments: [{
    //           hrl: JSON.stringify(hrlB64.hrl),
    //           context: JSON.stringify("attachToHrlWithContext.context")
    //         }]
    //       };
        
    //       console.log("createDeliberation", deliberationEntry)
    //       let record;

    //       try {
    //         record = await appletClient.callZome({
    //           cap_secret: null,
    //           role_name: 'converge',
    //           zome_name: 'converge',
    //           fn_name: 'create_deliberation',
    //           payload: deliberationEntry,
    //         });
        
    //         // join deliberation
    //         await appletClient.callZome({
    //           cap_secret: null,
    //           role_name: 'converge',
    //           zome_name: 'converge',
    //           fn_name: 'add_deliberation_for_deliberator',
    //           payload: {
    //             base_deliberator: appletClient.myPubKey,
    //             target_deliberation_hash: record.signed_action.hashed.hash
    //           },
    //         });
    //       } catch (e) {
    //         console.log(e)
    //       }

    //       console.log("hash", record.signed_action.hashed.hash)

    //       return {
    //         hrl: [dnaHash, record.signed_action.hashed.hash],
    //       };
    //     },
    //   },
    // }),
};