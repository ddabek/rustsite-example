///<reference path="../node_modules/angular2/typings/browser.d.ts"/>

import {bootstrap} from 'angular2/platform/browser';
import {Component} from 'angular2/core';
import {Http, Response} from "angular2/http";
import 'rxjs/add/operator/map';
import {HTTP_PROVIDERS} from "angular2/http";

@Component({
	selector: 'app',
	template: `


		<br>First Name: <input placeholder="Input your First Name" #firstname>
		<br>Last Name: <input placeholder="Input your Last Name" #lastname>
		<br><button class="small-btn float-right" (click)="sendName(firstname, lastname)">Send Name</button>
		<button (click)="getNames()">get names</button>

		<br>{{ what_happened }}
		
		<ul>
			<li *ngFor="let name of names">
				 {{ name }} 
			</li>
		</ul>
	`,
	directives: []
})

export class AppComponent {
	what_happened: string;
	jsonstr: string;
	names: string[] = [];

	constructor(private _http: Http) {}

	send_name(body) {
		return this._http.post('http://localhost:3100/send_name', body)
			.map(res => res.json())
	}

	get_names() {
		return this._http.get('http://localhost:3100/get_names')
			.map(res => res.json())
	}

	sendName(firstname, lastname) {
		var self = this;
		var first = "first";
		var last = "last";
		var json = {};
		json[first] = firstname.value;
		json[last] = lastname.value;
		self.send_name(JSON.stringify(json))
			.subscribe(
				data => self.what_happened = JSON.stringify(data),
       			error => self.what_happened = "error",
				() => console.log("finished import")
			);
	}

	getNames() {
		var self = this;
		this.get_names()
			.subscribe(
				data => {
				self.jsonstr = JSON.stringify(data)
				var json = JSON.parse(self.jsonstr);
				for (var i = 0; i < json.length; i++) {
					console.log(json.length);
					self.names.push(JSON.stringify(json[i]));
    			}

			},
       			error => self.what_happened = "error",
				() => console.log("finished import")
			);
	}
 	
}

bootstrap(AppComponent, [HTTP_PROVIDERS]);