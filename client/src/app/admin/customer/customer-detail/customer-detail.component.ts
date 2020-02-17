import { Component, OnInit } from '@angular/core';
import { Router, Route, ActivatedRoute, ParamMap } from '@angular/router';
import { switchMap } from 'rxjs/operators';
import { Customer, CustomerAddress } from 'src/app/class/customer';
import { Model } from 'src/app/class/model';
import { HttpClient } from '@angular/common/http';

@Component({
  selector: 'app-customer-detail',
  templateUrl: './customer-detail.component.html',
  styleUrls: ['./customer-detail.component.css']
})
export class CustomerDetailComponent implements OnInit {

  id: string = this.route.snapshot.paramMap.get("id");
  customer: Model<Customer> = new Model<Customer>(this.http, "/customer/" + this.id, new Customer("", null, "", "", new CustomerAddress("", "", ""), "", "", "", false, []));

  constructor(private http: HttpClient, private route: ActivatedRoute, private router: Router) {
  }

  ngOnInit() {
    new Model<Customer>(this.http, "/customer/" + this.id, new Customer("", null, "", "", new CustomerAddress("", "", ""), "", "", "", false, []));
    this.http.get<Customer>("/customer/" + this.id).subscribe((val) => this.customer.data = val);
  }

}
