import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { ActivatedRoute, Router } from '@angular/router';
import { DocumentNew, Document } from 'src/app/class/document';
import { Route } from '@angular/compiler/src/core';

@Component({
  selector: 'app-document-new',
  templateUrl: './document-new.component.html',
  styleUrls: ['./document-new.component.css']
})
export class DocumentNewComponent implements OnInit {

  id: string = this.route.snapshot.paramMap.get("id");
  document: DocumentNew = new DocumentNew();

  constructor(private http: HttpClient, private route: ActivatedRoute, private router: Router) { }

  ngOnInit() {
  }

  submit() {
    this.http.put<Document>("/document/" + this.id + "/new", this.document).subscribe((val) => {
      this.router.navigateByUrl("/folder/" + this.id);
    });
  }

}
