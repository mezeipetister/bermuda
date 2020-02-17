import { Component, OnInit } from '@angular/core';
import { CustomerNew } from 'src/app/class/customer-new';
import { HttpClient } from '@angular/common/http';
import { Model } from 'src/app/class/model';
import { Router } from '@angular/router';
import { Customer } from 'src/app/class/customer';

@Component({
  selector: 'app-new',
  templateUrl: './new.component.html',
  styleUrls: ['./new.component.css']
})
export class CustomerNewComponent implements OnInit {

  customer: CustomerNew = new CustomerNew();

  submit() {
    this.http.post<Customer>("/customer/new", this.customer).subscribe((resp) => this.router.navigateByUrl("/a/customer/" + resp.id));
  }

  constructor(private http: HttpClient, private router: Router) { }

  ngOnInit() {
  }

}
