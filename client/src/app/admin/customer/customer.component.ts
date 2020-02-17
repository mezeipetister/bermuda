import { Component, OnInit } from '@angular/core';
import { Customer } from 'src/app/class/customer';
import { HttpClient } from '@angular/common/http';
import { Pager } from 'src/app/class/pager';

@Component({
  selector: 'app-customer',
  templateUrl: './customer.component.html',
  styleUrls: ['./customer.component.sass']
})
export class CustomerComponent implements OnInit {
  filter: string = "";
  customers: Pager<Customer> = new Pager([], 10);
  buffer: Customer[] = null;

  filterSubmit() {
    /**
     * Filter data
     */
    this.customers.data = this.buffer.filter((c) =>
      c.name.toUpperCase().includes(this.filter.toUpperCase())
      || (c.address.zip.toUpperCase()
        + c.address.location.toUpperCase()
        + c.address.address.toUpperCase())
        .includes(this.filter.toUpperCase()));
    /**
     * Reset pagination
     */
    this.customers.navigate_to(1);
  }

  constructor(private http: HttpClient) { }

  ngOnInit() {
    this.http.get<Customer[]>("/customer/all").subscribe((val) => {
      this.customers.set_data(val);
      this.buffer = val;
    });
  }

}
